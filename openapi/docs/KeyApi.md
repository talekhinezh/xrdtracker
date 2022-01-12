# \KeyApi

All URIs are relative to *http://192.168.86.37:3333*

Method | HTTP request | Description
------------- | ------------- | -------------
[**key_list_post**](KeyApi.md#key_list_post) | **POST** /key/list | Get public keys
[**key_sign_post**](KeyApi.md#key_sign_post) | **POST** /key/sign | Sign transaction



## key_list_post

> crate::models::KeyListResponse key_list_post(key_list_request)
Get public keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_list_request** | [**KeyListRequest**](KeyListRequest.md) |  | [required] |

### Return type

[**crate::models::KeyListResponse**](KeyListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## key_sign_post

> crate::models::KeySignResponse key_sign_post(key_sign_request)
Sign transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_sign_request** | [**KeySignRequest**](KeySignRequest.md) |  | [required] |

### Return type

[**crate::models::KeySignResponse**](KeySignResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

