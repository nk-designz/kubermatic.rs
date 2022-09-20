# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_to_project**](UsersApi.md#add_user_to_project) | **POST** /api/v1/projects/{project_id}/users | 
[**delete_user_from_project**](UsersApi.md#delete_user_from_project) | **DELETE** /api/v1/projects/{project_id}/users/{user_id} | 
[**edit_user_in_project**](UsersApi.md#edit_user_in_project) | **PUT** /api/v1/projects/{project_id}/users/{user_id} | 
[**get_current_user**](UsersApi.md#get_current_user) | **GET** /api/v1/me | Returns information about the current user.
[**get_users_for_project**](UsersApi.md#get_users_for_project) | **GET** /api/v1/projects/{project_id}/users | 
[**logout_current_user**](UsersApi.md#logout_current_user) | **POST** /api/v1/me/logout | Adds current authorization bearer token to the blacklist.



## add_user_to_project

> crate::models::User add_user_to_project(project_id, body)


Adds the given user to the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**body** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_from_project

> crate::models::User delete_user_from_project(project_id, user_id)


Removes the given member from the project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_user_in_project

> crate::models::User edit_user_in_project(project_id, user_id, body)


Changes membership of the given user for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**body** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> crate::models::User get_current_user()
Returns information about the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](User.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_for_project

> Vec<crate::models::User> get_users_for_project(project_id)


Get list of users for the given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_current_user

> logout_current_user()
Adds current authorization bearer token to the blacklist.

Enforces user to login again with the new token.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

