# \EntityApi

All URIs are relative to *http://192.168.86.37:3333*

Method | HTTP request | Description
------------- | ------------- | -------------
[**entity_post**](EntityApi.md#entity_post) | **POST** /entity | Get Entity Information



## entity_post

> crate::models::EntityResponse entity_post(entity_request)
Get Entity Information

Gets the balances and data objects at an entity at the current state of the ledger.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_request** | [**EntityRequest**](EntityRequest.md) |  | [required] |

### Return type

[**crate::models::EntityResponse**](EntityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

