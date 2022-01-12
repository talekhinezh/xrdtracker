/*
 * Radix Core API
 *
 * This API provides endpoints from a node for integration with the Radix ledger.  # Overview  > WARNING > > The Core API is __NOT__ intended to be available on the public web. It is > designed to be accessed in a private network.  The Core API is separated into three: * The **Data API** is a read-only api which allows you to view and sync to the state of the ledger. * The **Construction API** allows you to construct and submit a transaction to the network. * The **Key API** allows you to use the keys managed by the node to sign transactions.  The Core API is a low level API primarily designed for network integrations such as exchanges, ledger analytics providers, or hosted ledger data dashboards where detailed ledger data is required and the integrator can be expected to run their node to provide the Core API for their own consumption.  For a higher level API, see the [Gateway API](https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/radixdlt/radixdlt-network-gateway/develop/generation/gateway-api-spec.yaml).  For node monitoring, see the [System API](https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/radixdlt/radixdlt/develop/radixdlt-core/radixdlt/src/main/java/com/radixdlt/api/system/api.yaml).  ## Rosetta  The Data API and Construction API is inspired from [Rosetta API](https://www.rosetta-api.org/) most notably:   * Use of a JSON-Based RPC protocol on top of HTTP Post requests   * Use of Operations, Amounts, and Identifiers as universal language to   express asset movement for reading and writing  There are a few notable exceptions to note:   * Fetching of ledger data is through a Transaction stream rather than a   Block stream   * Use of `EntityIdentifier` rather than `AccountIdentifier`   * Use of `OperationGroup` rather than `related_operations` to express related   operations   * Construction endpoints perform coin selection on behalf of the caller.   This has the unfortunate effect of not being able to support high frequency   transactions from a single account. This will be addressed in future updates.   * Construction endpoints are online rather than offline as required by Rosetta  Future versions of the api will aim towards a fully-compliant Rosetta API.  ## Enabling Endpoints  All endpoints are enabled when running a node with the exception of two endpoints, each of which need to be manually configured to access: * `/transactions` endpoint must be enabled with configuration `api.transaction.enable=true`. This is because the transactions endpoint requires additional database storage which may not be needed for users who aren't using this endpoint * `/key/sign` endpoint must be enable with configuration `api.sign.enable=true`. This is a potentially dangerous endpoint if accessible publicly so it must be enabled manually.  ## Client Code Generation  We have found success with generating clients against the [api.yaml specification](https://raw.githubusercontent.com/radixdlt/radixdlt/develop/radixdlt-core/radixdlt/src/main/java/com/radixdlt/api/core/api.yaml). See https://openapi-generator.tech/ for more details.  The OpenAPI generator only supports openapi version 3.0.0 at present, but you can change 3.1.0 to 3.0.0 in the first line of the spec without affecting generation.  # Data API Flow  The Data API can be used to synchronize a full or partial view of the ledger, transaction by transaction.  ![Data API Flow](https://raw.githubusercontent.com/radixdlt/radixdlt/feature/update-documentation/radixdlt-core/radixdlt/src/main/java/com/radixdlt/api/core/documentation/data_sequence_flow.png)  # Construction API Flow  The Construction API can be used to construct and submit transactions to the network.  ![Construction API Flow](https://raw.githubusercontent.com/radixdlt/radixdlt/feature/open-api/radixdlt-core/radixdlt/src/main/java/com/radixdlt/api/core/documentation/construction_sequence_flow.png)  Unlike the Rosetta Construction API [specification](https://www.rosetta-api.org/docs/construction_api_introduction.html), this Construction API selects UTXOs on behalf of the caller. This has the unfortunate side effect of not being able to support high frequency transactions from a single account due to UTXO conflicts. This will be addressed in a future release. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DataObject {
    #[serde(rename="EpochData")]
    EpochData {
        #[serde(rename = "epoch")]
        epoch: i64,
    },
    #[serde(rename="PreparedValidatorFee")]
    PreparedValidatorFee {
        /// The fee percentage of the validator which will be updated in the future epoch.
        #[serde(rename = "fee")]
        fee: i32,
        #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
        epoch: Option<i64>,
    },
    #[serde(rename="PreparedValidatorOwner")]
    PreparedValidatorOwner {
        #[serde(rename = "owner")]
        owner: Box<crate::models::EntityIdentifier>,
        #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
        epoch: Option<i64>,
    },
    #[serde(rename="PreparedValidatorRegistered")]
    PreparedValidatorRegistered {
        /// The registered flag of the validator which will be updated by the end of `epoch`.
        #[serde(rename = "registered")]
        registered: bool,
        #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
        epoch: Option<i64>,
    },
    #[serde(rename="RoundData")]
    RoundData {
        #[serde(rename = "round")]
        round: i64,
        #[serde(rename = "timestamp")]
        timestamp: i64,
    },
    #[serde(rename="TokenData")]
    TokenData {
        #[serde(rename = "granularity")]
        granularity: String,
        /// If true, the `owner` is able to mint/burn tokens. Otherwise, the token is a fixed supply token.
        #[serde(rename = "is_mutable")]
        is_mutable: bool,
        #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
        owner: Option<Box<crate::models::EntityIdentifier>>,
    },
    #[serde(rename="TokenMetadata")]
    TokenMetadata {
        /// The symbol of the token (not unique in the system).
        #[serde(rename = "symbol")]
        symbol: String,
        /// The name of the token.
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Description describing the token.
        #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// A url which points to more information about the token.
        #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        /// A url which points to the icon of the token.
        #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
        icon_url: Option<String>,
    },
    #[serde(rename="UnclaimedRadixEngineAddress")]
    UnclaimedRadixEngineAddress {
    },
    #[serde(rename="ValidatorAllowDelegation")]
    ValidatorAllowDelegation {
        /// Flag indicating whether stakers besides the owner of the validator can stake to this validator.
        #[serde(rename = "allow_delegation")]
        allow_delegation: bool,
    },
    #[serde(rename="ValidatorBFTData")]
    ValidatorBftData {
        #[serde(rename = "proposals_completed")]
        proposals_completed: i64,
        #[serde(rename = "proposals_missed")]
        proposals_missed: i64,
    },
    #[serde(rename="ValidatorData")]
    ValidatorData {
        #[serde(rename = "owner")]
        owner: Box<crate::models::EntityIdentifier>,
        /// Flag indicating whether a validator is registered to be part of the validator set.
        #[serde(rename = "registered")]
        registered: bool,
        /// The fee percentage of the validator in 0.01% subunits (ie 10000 == 100%).
        #[serde(rename = "fee")]
        fee: i32,
    },
    #[serde(rename="ValidatorMetadata")]
    ValidatorMetadata {
        /// The name for the validator.
        #[serde(rename = "name")]
        name: String,
        /// A url which points to more information about the validator.
        #[serde(rename = "url")]
        url: String,
    },
    #[serde(rename="ValidatorSystemMetadata")]
    ValidatorSystemMetadata {
        /// A hex encoded byte array.
        #[serde(rename = "data")]
        data: String,
    },
    #[serde(rename="VirtualParentData")]
    VirtualParentData {
        #[serde(rename = "entity_set_identifier")]
        entity_set_identifier: Box<crate::models::EntitySetIdentifier>,
        #[serde(rename = "virtual_data_object")]
        virtual_data_object: Box<crate::models::DataObject>,
    },
}




