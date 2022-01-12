# \EngineApi

All URIs are relative to *http://192.168.86.37:3333*

Method | HTTP request | Description
------------- | ------------- | -------------
[**engine_configuration_post**](EngineApi.md#engine_configuration_post) | **POST** /engine/configuration | Get Engine Configuration
[**engine_status_post**](EngineApi.md#engine_status_post) | **POST** /engine/status | Get Engine Current Status



## engine_configuration_post

> crate::models::EngineConfigurationResponse engine_configuration_post(engine_configuration_request)
Get Engine Configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**engine_configuration_request** | [**EngineConfigurationRequest**](EngineConfigurationRequest.md) |  | [required] |

### Return type

[**crate::models::EngineConfigurationResponse**](EngineConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## engine_status_post

> crate::models::EngineStatusResponse engine_status_post(engine_status_request)
Get Engine Current Status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**engine_status_request** | [**EngineStatusRequest**](EngineStatusRequest.md) |  | [required] |

### Return type

[**crate::models::EngineStatusResponse**](EngineStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

