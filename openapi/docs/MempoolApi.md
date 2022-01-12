# \MempoolApi

All URIs are relative to *http://192.168.86.37:3333*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mempool_post**](MempoolApi.md#mempool_post) | **POST** /mempool | Get Mempool Transactions
[**mempool_transaction_post**](MempoolApi.md#mempool_transaction_post) | **POST** /mempool/transaction | Get Mempool Transaction



## mempool_post

> crate::models::MempoolResponse mempool_post(mempool_request)
Get Mempool Transactions

Gets the transaction identifiers in the mempool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mempool_request** | [**MempoolRequest**](MempoolRequest.md) |  | [required] |

### Return type

[**crate::models::MempoolResponse**](MempoolResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mempool_transaction_post

> crate::models::MempoolTransactionResponse mempool_transaction_post(mempool_transaction_request)
Get Mempool Transaction

Gets the transaction from the mempool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mempool_transaction_request** | [**MempoolTransactionRequest**](MempoolTransactionRequest.md) |  | [required] |

### Return type

[**crate::models::MempoolTransactionResponse**](MempoolTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

