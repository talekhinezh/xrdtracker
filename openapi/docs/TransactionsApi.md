# \TransactionsApi

All URIs are relative to *http://192.168.86.37:3333*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transactions_post**](TransactionsApi.md#transactions_post) | **POST** /transactions | Get Committed Transactions



## transactions_post

> crate::models::CommittedTransactionsResponse transactions_post(committed_transactions_request)
Get Committed Transactions

Returns an ordered sublist of committed transactions. This endpoint is designed for lite clients to sync with the state of the ledger.  The example response demonstrates a transfer transaction.  There is a more detailed worked example of reading this endpoint in the examples section. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**committed_transactions_request** | [**CommittedTransactionsRequest**](CommittedTransactionsRequest.md) |  | [required] |

### Return type

[**crate::models::CommittedTransactionsResponse**](CommittedTransactionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

