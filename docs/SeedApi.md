# \SeedApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_seed_settings**](SeedApi.md#get_seed_settings) | **GET** /api/v2/seeds/{seed_name}/settings | Gets the seed settings.
[**list_seed_names**](SeedApi.md#list_seed_names) | **GET** /api/v1/seed | 



## get_seed_settings

> crate::models::SeedSettings get_seed_settings(seed_name)
Gets the seed settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |

### Return type

[**crate::models::SeedSettings**](SeedSettings.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_seed_names

> crate::models::SeedNamesList list_seed_names()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SeedNamesList**](SeedNamesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

