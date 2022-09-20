# \VersionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_versions_by_provider**](VersionApi.md#list_versions_by_provider) | **GET** /api/v2/providers/{provider_name}/versions | 



## list_versions_by_provider

> crate::models::VersionList list_versions_by_provider(provider_name, _type)


Lists all versions which don't result in automatic updates for a given provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**_type** | Option<**String**> | Type is deprecated and not used anymore. |  |

### Return type

[**crate::models::VersionList**](VersionList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

