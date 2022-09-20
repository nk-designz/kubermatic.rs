# \CniversionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_versions_by_cni_plugin**](CniversionApi.md#list_versions_by_cni_plugin) | **GET** /api/v2/cni/{cni_plugin_type}/versions | 



## list_versions_by_cni_plugin

> crate::models::CniVersions list_versions_by_cni_plugin(cni_plugin_type)


Lists all CNI Plugin versions that are supported for a given CNI plugin type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cni_plugin_type** | **String** |  | [required] |

### Return type

[**crate::models::CniVersions**](CNIVersions.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

