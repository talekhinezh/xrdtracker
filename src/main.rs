use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use futures::{StreamExt};
use serde_json::json;
use openapi::apis::*;
use openapi::models::{CommittedTransactionsRequest, DataObject, PartialStateIdentifier, ResourceIdentifier};
use mongodb::{Client, Database, options::ClientOptions};
use mongodb::bson::{doc, DateTime, from_document};
use mongodb::options::{FindOneOptions};
use num_bigint::{BigInt};
use num_traits::identities::Zero;
use num_traits::Signed;
use num_integer::Integer;
use openapi::models::data::Action::CREATE;
use serde::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BalanceEntry {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(rename = "datetime")]
    pub datetime: DateTime
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SyncEntry {
    #[serde(rename = "version")]
    pub version: i64
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TimeEntry {
    #[serde(rename = "epoch")]
    pub epoch: i64,
    #[serde(rename = "round")]
    pub round: i64,
    #[serde(rename = "timestamp")]
    pub timestamp: i64
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct DateTimeBalance {
    #[serde(rename = "datetime")]
    pub datetime: DateTime,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "balance")]
    pub balance: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct DateTimeStake {
    #[serde(rename = "datetime")]
    pub datetime: DateTime,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "balance")]
    pub balance: String
}

async fn db_client() -> Client {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("Oops");
    let client = Client::with_options(client_options).expect("Oops");
    client
}

async fn update() -> Result<i64, ()> {
    let config = configuration::Configuration::default();
    let value = json!({});
    let response = network_api::network_configuration_post(&config, value);
    let result = response.await.expect("Failed");
    let network_identifier = *result.network_identifier;

    let client = db_client().await;
    let db = client.database("radix");
    let balance_collection = db.collection::<BalanceEntry>("balances");
    let sync_collection = db.collection::<SyncEntry>("sync");
    let time_collection = db.collection::<TimeEntry>("time");

    let mut session = client.start_session(None).await.expect("Oops");
    session.start_transaction(None).await.expect("Oops");
    let sync_entry = sync_collection.find_one_and_delete_with_session(doc! { }, None, &mut session).await.expect("oops");

    let next_version = sync_entry.map_or(0, |e| e.version);
    let partial_state_identifier = PartialStateIdentifier::new(next_version);
    let mut request = CommittedTransactionsRequest::new(network_identifier.clone(), partial_state_identifier);
    request.limit = Option::Some(1000);
    let transactions_response = transactions_api::transactions_post(&config, request).await.expect("Failed");

    let num_txns = transactions_response.transactions.len() as i64;
    let ending_version = next_version + num_txns;
    sync_collection.insert_one_with_session(SyncEntry {
        version: ending_version
    }, None, &mut session).await.expect("Oops");

    let mut time_entry = time_collection.find_one_with_session(doc! {}, None, &mut session).await.expect("oops")
        .unwrap_or(TimeEntry { epoch: 0, timestamp: 0, round: 0});

    for (index, txn) in transactions_response.transactions.iter().enumerate() {
        let iter = txn.operation_groups.iter()
            .flat_map(|group| group.operations.iter())
            .flat_map(|op| op.data.clone())
            .filter(|data| data.action.eq(&CREATE))
            .map(|data| data.data_object);
        for obj in iter {
            match *obj {
                DataObject::EpochData { epoch } => {
                    time_entry.epoch = epoch;
                }
                DataObject::RoundData { round, timestamp } => {
                    time_entry.timestamp = timestamp;
                    time_entry.round = round;
                }
                _ => {}
            }
        }

        let mut balance_changes: HashMap<(String, String), BigInt> = HashMap::new();
        txn.operation_groups.iter()
            .flat_map(|group| group.operations.iter())
            .filter(|op| op.amount.is_some())
            .map(|op| (
                *op.entity_identifier.clone(),
                *op.amount.clone().unwrap().resource_identifier,
                op.amount.clone().unwrap().value.parse::<BigInt>().expect("oops")
            ))
            .for_each(|change| -> () {
                let asset = match change.1.clone() {
                    ResourceIdentifier::StakeUnitResourceIdentifier { validator_address } => String::from(format!("stake_{}", validator_address)),
                    ResourceIdentifier::TokenResourceIdentifier { rri } => rri
                };
                let record = balance_changes.entry((change.0.address, asset)).or_insert(BigInt::zero());
                record.add_assign(change.2);
            });

        for balance_change in balance_changes.iter() {
            if balance_change.1.is_zero() {
                continue;
            }

            let address = balance_change.0.0.clone();
            let asset = balance_change.0.1.clone();

            let find_options = FindOneOptions::builder().sort(doc! { "version": -1 }).build();
            let doc = doc! { "address": address.clone(), "asset": asset.clone() };
            let last_balance = balance_collection.find_one_with_session(doc, find_options, &mut session)
                .await.expect("oops").map_or(BigInt::zero(), |b| b.balance.parse::<BigInt>().expect("oops"));
            let next_balance = last_balance.clone().add(balance_change.1);
            if next_balance.is_negative() {
                panic!("next_balance is negative: {} {} {:?} {:?}", address, asset, last_balance, balance_change.1);
            }
            let balance_entry = BalanceEntry {
                address: address.clone(),
                asset: asset.clone(),
                balance: next_balance.to_string(),
                version: next_version + index as i64,
                datetime: DateTime::from_millis(time_entry.timestamp)
            };
            println!("{} {} {:?} -> {:?}", address, asset, last_balance, next_balance);
            balance_collection.insert_one_with_session(balance_entry, None, &mut session).await.expect("oops");
        }
    }
    let update = doc! {
        "$set": {
            "epoch": time_entry.epoch,
            "round": time_entry.round,
            "timestamp": time_entry.timestamp,
        }
    };
    let u = time_collection.update_one_with_session(doc! {}, update, None, &mut session).await.expect("oops");
    if u.matched_count == 0 {
        time_collection.insert_one_with_session(time_entry.clone(), None, &mut session).await.expect("oops");
    }

    println!("epoch {} round {} version {}", time_entry.epoch, time_entry.round, ending_version);
    session.commit_transaction().await.expect("oops");

    Ok(num_txns)
}

async fn total_stake(validator: &String, date_time: &DateTime, db: &mut Database) -> BigInt {
    let balances_collection = db.collection::<BalanceEntry>("balances");
    let pipeline = vec![
        doc! {
            "$sort": {
                "version": 1,
                "address": 1,
                "asset": 1
            }
        },
        doc! {
            "$match": {
              "address": validator,
              "asset": "xrd_rr1qy5wfsfh",
              "datetime": {
                "$lt": date_time
              }
            }
        },
        doc! {
        "$group": {
          "_id": {
            "address": "$address",
            "asset": "$asset"
          },
          "datetime": {
            "$last": "$datetime"
          },
          "address": {
            "$last": "$address"
          },
          "balance": {
            "$last": "$balance"
          }
        }
      }
    ];
    let mut results = balances_collection.aggregate(pipeline, None).await.expect("oops");
    let mut total_units = BigInt::zero();
    while let Some(result) = results.next().await {
        let doc: DateTimeStake = from_document(result.expect("oops")).expect("oops");
        let unit = doc.balance.parse::<BigInt>().expect("oops");
        total_units = total_units + unit;
    }

    total_units
}

async fn total_units(stake_asset: &String, date_time: &DateTime, db: &mut Database) -> BigInt {
    let balances_collection = db.collection::<BalanceEntry>("balances");
    let pipeline = vec![
        doc! {
            "$sort": {
                "version": 1,
                "address": 1,
                "asset": 1
            }
        },
        doc! {
            "$match": {
              "asset": stake_asset,
              "datetime": {
                "$lt": date_time
              }
            }
        },
        doc! {
        "$group": {
          "_id": {
            "address": "$address",
            "asset": "$asset"
          },
          "datetime": {
            "$last": "$datetime"
          },
          "address": {
            "$last": "$address"
          },
          "balance": {
            "$last": "$balance"
          }
        }
      }
    ];
    let mut results = balances_collection.aggregate(pipeline, None).await.expect("oops");
    let mut total_units = BigInt::zero();
    while let Some(result) = results.next().await {
        let doc: DateTimeStake = from_document(result.expect("oops")).expect("oops");
        let unit = doc.balance.parse::<BigInt>().expect("oops");
        total_units = total_units + unit;
    }

    total_units
}

async fn address(address: &String) {
    let client = db_client().await;
    let mut db = client.database("radix");
    let balances_collection = db.collection::<BalanceEntry>("balances");
    let datetime = DateTime::parse_rfc3339_str("2021-12-31T23:59:59.999-06:00").expect("invalid date");
    let pipeline = vec![
        doc! {
            "$sort": {
                "version": 1,
                "address": 1,
                "asset": 1
            }
        },
        doc! {
            "$match": {
              "address": address,
              "datetime": {
                "$lte": datetime
              }
            }
        },
        doc! {
        "$group": {
          "_id": {
            "address": "$address",
            "asset": "$asset"
          },
          "datetime": {
            "$last": "$datetime"
          },
          "asset": {
            "$last": "$asset"
          },
          "balance": {
            "$last": "$balance"
          }
        }
      }
    ];
    println!("address: {}", address);
    println!("date: {}", datetime);
    let mut xrd = BigInt::zero();
    let granularity = BigInt::from(10).pow(18);
    let mut total_staked_xrd = BigInt::zero();
    let mut results = balances_collection.aggregate(pipeline, None).await.expect("oops");
    while let Some(result) = results.next().await {
        let doc: DateTimeBalance = from_document(result.expect("oops")).expect("oops");
        if doc.asset.starts_with("stake_") {
            let validator = doc.asset.split_at(6).1.clone();
            let total_stake = total_stake(&validator.to_string(), &datetime, &mut db).await;
            let stake_units = doc.balance.parse::<BigInt>().expect("oops");
            let total_units = total_units(&doc.asset, &datetime, &mut db).await;
            let staked_xrd = stake_units * total_stake / total_units;
            total_staked_xrd += staked_xrd;
        } else if doc.asset.eq("xrd_rr1qy5wfsfh") {
            xrd = doc.balance.parse::<BigInt>().expect("oops");
        } else {
            let (amt, rem) = doc.balance.parse::<BigInt>().expect("oops").div_rem(&granularity);
            println!("{}: {}.{:0>18}", doc.asset, amt, rem);
        }
    }

    let (amt, rem) = xrd.div_rem(&granularity);
    println!("xrd_rr1qy5wfsfh (liquid): {}.{:0>18}", amt, rem);
    if total_staked_xrd.is_positive() {
        let (amt, rem) = total_staked_xrd.div_rem(&granularity);
        println!("xrd_rr1qy5wfsfh (staked): {}.{:0>18}", amt, rem);
    }
    let total_xrd = xrd + total_staked_xrd;
    let (amt, rem) = total_xrd.div_rem(&granularity);
    println!("xrd_rr1qy5wfsfh (total): {}.{:0>18}", amt, rem);
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    /*
    loop {
        if update().await? == 0 {
            break;
        }
    }
     */
    let args: Vec<String> = std::env::args().collect();
    let account_addr = &args[1];
    address(account_addr).await;
    Ok(())
}
