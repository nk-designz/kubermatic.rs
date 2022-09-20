# \ServiceaccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_service_account_to_project**](ServiceaccountsApi.md#add_service_account_to_project) | **POST** /api/v1/projects/{project_id}/serviceaccounts | 
[**delete_service_account**](ServiceaccountsApi.md#delete_service_account) | **DELETE** /api/v1/projects/{project_id}/serviceaccounts/{serviceaccount_id} | 
[**list_service_accounts**](ServiceaccountsApi.md#list_service_accounts) | **GET** /api/v1/projects/{project_id}/serviceaccounts | 
[**update_service_account**](ServiceaccountsApi.md#update_service_account) | **PUT** /api/v1/projects/{project_id}/serviceaccounts/{serviceaccount_id} | 



## add_service_account_to_project

> crate::models::ServiceAccount add_service_account_to_project(project_id, body)


Adds the given service account to the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | Option<[**ServiceAccount**](ServiceAccount.md)> |  |  |

### Return type

[**crate::models::ServiceAccount**](ServiceAccount.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_account

> delete_service_account(project_id, serviceaccount_id)


Deletes service account for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**serviceaccount_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_accounts

> Vec<crate::models::ServiceAccount> list_service_accounts(project_id)


List Service Accounts for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ServiceAccount>**](ServiceAccount.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_account

> crate::models::ServiceAccount update_service_account(project_id, serviceaccount_id, body)


Updates service account for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**serviceaccount_id** | **String** |  | [required] |
**body** | Option<[**ServiceAccount**](ServiceAccount.md)> |  |  |

### Return type

[**crate::models::ServiceAccount**](ServiceAccount.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

