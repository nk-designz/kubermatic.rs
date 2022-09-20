# \NetworkdefaultsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_network_defaults**](NetworkdefaultsApi.md#get_network_defaults) | **GET** /providers/{provider_name}/dc/{dc}/networkdefaults | Retrieves the cluster networking defaults for the given provider and datacenter.



## get_network_defaults

> crate::models::NetworkDefaults get_network_defaults(provider_name, dc)
Retrieves the cluster networking defaults for the given provider and datacenter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**dc** | **String** |  | [required] |

### Return type

[**crate::models::NetworkDefaults**](NetworkDefaults.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

