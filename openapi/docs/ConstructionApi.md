# \ConstructionApi

All URIs are relative to *http://192.168.86.37:3333*

Method | HTTP request | Description
------------- | ------------- | -------------
[**construction_build_post**](ConstructionApi.md#construction_build_post) | **POST** /construction/build | Build Transaction
[**construction_derive_post**](ConstructionApi.md#construction_derive_post) | **POST** /construction/derive | Derive Entity Identifier
[**construction_finalize_post**](ConstructionApi.md#construction_finalize_post) | **POST** /construction/finalize | Finalize Transaction
[**construction_hash_post**](ConstructionApi.md#construction_hash_post) | **POST** /construction/hash | Get Transaction Hash
[**construction_parse_post**](ConstructionApi.md#construction_parse_post) | **POST** /construction/parse | Parse Transaction
[**construction_submit_post**](ConstructionApi.md#construction_submit_post) | **POST** /construction/submit | Submit Transaction



## construction_build_post

> crate::models::ConstructionBuildResponse construction_build_post(construction_build_request)
Build Transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**construction_build_request** | [**ConstructionBuildRequest**](ConstructionBuildRequest.md) |  | [required] |

### Return type

[**crate::models::ConstructionBuildResponse**](ConstructionBuildResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## construction_derive_post

> crate::models::ConstructionDeriveResponse construction_derive_post(construction_derive_request)
Derive Entity Identifier

Returns the entity identifier for an account, validator, or token given a public key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**construction_derive_request** | [**ConstructionDeriveRequest**](ConstructionDeriveRequest.md) |  | [required] |

### Return type

[**crate::models::ConstructionDeriveResponse**](ConstructionDeriveResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## construction_finalize_post

> crate::models::ConstructionFinalizeResponse construction_finalize_post(construction_finalize_request)
Finalize Transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**construction_finalize_request** | [**ConstructionFinalizeRequest**](ConstructionFinalizeRequest.md) |  | [required] |

### Return type

[**crate::models::ConstructionFinalizeResponse**](ConstructionFinalizeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## construction_hash_post

> crate::models::ConstructionHashResponse construction_hash_post(construction_hash_request)
Get Transaction Hash

Get the transaction identifier of a signed transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**construction_hash_request** | [**ConstructionHashRequest**](ConstructionHashRequest.md) |  | [required] |

### Return type

[**crate::models::ConstructionHashResponse**](ConstructionHashResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## construction_parse_post

> crate::models::ConstructionParseResponse construction_parse_post(construction_parse_request)
Parse Transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**construction_parse_request** | [**ConstructionParseRequest**](ConstructionParseRequest.md) |  | [required] |

### Return type

[**crate::models::ConstructionParseResponse**](ConstructionParseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## construction_submit_post

> crate::models::ConstructionSubmitResponse construction_submit_post(construction_submit_request)
Submit Transaction

Submit a transaction to the mempool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**construction_submit_request** | [**ConstructionSubmitRequest**](ConstructionSubmitRequest.md) |  | [required] |

### Return type

[**crate::models::ConstructionSubmitResponse**](ConstructionSubmitResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

