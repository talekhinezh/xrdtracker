# ConstructionBuildRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_identifier** | [**crate::models::NetworkIdentifier**](NetworkIdentifier.md) |  | 
**operation_groups** | [**Vec<crate::models::OperationGroup>**](OperationGroup.md) | Operation groups which describe the intent of the request. | 
**fee_payer** | [**crate::models::EntityIdentifier**](EntityIdentifier.md) |  | 
**message** | Option<**String**> | An optional message payload encoded in hex with the Radix message encoding scheme. | [optional]
**disable_resource_allocate_and_destroy** | Option<**bool**> | Disallow minting and burning of tokens (except for fees). Useful for verification of transactions without needing to fetch additional substate data, such as when verifying transactions in an offline environment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


