# \TokensApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_token_to_service_account**](TokensApi.md#add_token_to_service_account) | **POST** /api/v1/projects/{project_id}/serviceaccounts/{serviceaccount_id}/tokens | 
[**delete_service_account_token**](TokensApi.md#delete_service_account_token) | **DELETE** /api/v1/projects/{project_id}/serviceaccounts/{serviceaccount_id}/tokens/{token_id} | 
[**list_service_account_tokens**](TokensApi.md#list_service_account_tokens) | **GET** /api/v1/projects/{project_id}/serviceaccounts/{serviceaccount_id}/tokens | 
[**patch_service_account_token**](TokensApi.md#patch_service_account_token) | **PATCH** /api/v1/projects/{project_id}/serviceaccounts/{serviceaccount_id}/tokens/{token_id} | 
[**update_service_account_token**](TokensApi.md#update_service_account_token) | **PUT** /api/v1/projects/{project_id}/serviceaccounts/{serviceaccount_id}/tokens/{token_id} | 



## add_token_to_service_account

> crate::models::ServiceAccountToken add_token_to_service_account(project_id, serviceaccount_id, body)


Generates a token for the given service account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**serviceaccount_id** | **String** |  | [required] |
**body** | Option<[**ServiceAccountToken**](ServiceAccountToken.md)> |  |  |

### Return type

[**crate::models::ServiceAccountToken**](ServiceAccountToken.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_account_token

> delete_service_account_token(project_id, serviceaccount_id, token_id)


Deletes the token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**serviceaccount_id** | **String** |  | [required] |
**token_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_account_tokens

> Vec<crate::models::PublicServiceAccountToken> list_service_account_tokens(project_id, serviceaccount_id)


List tokens for the given service account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**serviceaccount_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PublicServiceAccountToken>**](PublicServiceAccountToken.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_service_account_token

> crate::models::PublicServiceAccountToken patch_service_account_token(project_id, serviceaccount_id, token_id, body)


Patches the token name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**serviceaccount_id** | **String** |  | [required] |
**token_id** | **String** |  | [required] |
**body** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**crate::models::PublicServiceAccountToken**](PublicServiceAccountToken.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_account_token

> crate::models::ServiceAccountToken update_service_account_token(project_id, serviceaccount_id, token_id, body)


Updates and regenerates the token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**serviceaccount_id** | **String** |  | [required] |
**token_id** | **String** |  | [required] |
**body** | Option<[**PublicServiceAccountToken**](PublicServiceAccountToken.md)> |  |  |

### Return type

[**crate::models::ServiceAccountToken**](ServiceAccountToken.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

