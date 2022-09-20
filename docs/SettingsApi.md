# \SettingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_current_user_settings**](SettingsApi.md#get_current_user_settings) | **GET** /api/v1/me/settings | Returns settings of the current user.
[**patch_current_user_settings**](SettingsApi.md#patch_current_user_settings) | **PATCH** /api/v1/me/settings | Updates settings of the current user.



## get_current_user_settings

> crate::models::UserSettings get_current_user_settings()
Returns settings of the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserSettings**](UserSettings.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_current_user_settings

> crate::models::UserSettings patch_current_user_settings(patch)
Updates settings of the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::UserSettings**](UserSettings.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

