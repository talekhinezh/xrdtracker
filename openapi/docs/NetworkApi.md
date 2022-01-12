# \NetworkApi

All URIs are relative to *http://192.168.86.37:3333*

Method | HTTP request | Description
------------- | ------------- | -------------
[**network_configuration_post**](NetworkApi.md#network_configuration_post) | **POST** /network/configuration | Get Network Configuration
[**network_status_post**](NetworkApi.md#network_status_post) | **POST** /network/status | Get Network Status



## network_configuration_post

> crate::models::NetworkConfigurationResponse network_configuration_post(body)
Get Network Configuration

Returns the network configuration of the network the node is connected to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::NetworkConfigurationResponse**](NetworkConfigurationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_status_post

> crate::models::NetworkStatusResponse network_status_post(network_status_request)
Get Network Status

Returns the current state and status of the node's copy of the ledger. If the node is syncing, the `current_state_X` responses may be behind the global ledger.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_status_request** | [**NetworkStatusRequest**](NetworkStatusRequest.md) |  | [required] |

### Return type

[**crate::models::NetworkStatusResponse**](NetworkStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

