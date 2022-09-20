# \BackupcredentialsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_backup_credentials**](BackupcredentialsApi.md#create_or_update_backup_credentials) | **PUT** /api/v2/seeds/{seed_name}/backupcredentials | 



## create_or_update_backup_credentials

> create_or_update_backup_credentials(seed_name, body)


Creates or updates backup credentials for a given seed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**body** | Option<[**BcBody**](BcBody.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

