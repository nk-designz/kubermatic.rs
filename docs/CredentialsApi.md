# \CredentialsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_credentials**](CredentialsApi.md#list_credentials) | **GET** /api/v1/providers/{provider_name}/presets/credentials | 



## list_credentials

> crate::models::CredentialList list_credentials(provider_name, datacenter)


Lists credential names for the provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**datacenter** | Option<**String**> |  |  |

### Return type

[**crate::models::CredentialList**](CredentialList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

