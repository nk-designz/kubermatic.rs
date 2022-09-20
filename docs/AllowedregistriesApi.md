# \AllowedregistriesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_allowed_registry**](AllowedregistriesApi.md#delete_allowed_registry) | **DELETE** /api/v2/allowedregistries/{allowed_registry} | Deletes the given allowed registry.
[**get_allowed_registry**](AllowedregistriesApi.md#get_allowed_registry) | **GET** /api/v2/allowedregistries/{allowed_registry} | 
[**patch_allowed_registry**](AllowedregistriesApi.md#patch_allowed_registry) | **PATCH** /api/v2/allowedregistries/{allowed_registry} | 



## delete_allowed_registry

> delete_allowed_registry(allowed_registry)
Deletes the given allowed registry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allowed_registry** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_allowed_registry

> crate::models::AllowedRegistry get_allowed_registry(allowed_registry)


Get allowed registries specified by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allowed_registry** | **String** |  | [required] |

### Return type

[**crate::models::AllowedRegistry**](AllowedRegistry.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_allowed_registry

> crate::models::ConstraintTemplate patch_allowed_registry(allowed_registry, patch)


Patch a specified allowed registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allowed_registry** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::ConstraintTemplate**](ConstraintTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

