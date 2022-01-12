# Operation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | The type of operation: Resource, Data, or ResourceAndData. | 
**entity_identifier** | [**crate::models::EntityIdentifier**](EntityIdentifier.md) |  | 
**substate** | Option<[**crate::models::Substate**](Substate.md)> |  | [optional]
**amount** | Option<[**crate::models::ResourceAmount**](ResourceAmount.md)> |  | [optional]
**data** | Option<[**crate::models::Data**](Data.md)> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata for the operation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


