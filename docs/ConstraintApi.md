# \ConstraintApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_default_constraint**](ConstraintApi.md#create_default_constraint) | **POST** /api/v2/constraints | 
[**get_default_constraint**](ConstraintApi.md#get_default_constraint) | **GET** /api/v2/constraints/{constraint_name} | 
[**list_default_constraint**](ConstraintApi.md#list_default_constraint) | **GET** /api/v2/constraints | List default constraint.
[**patch_default_constraint**](ConstraintApi.md#patch_default_constraint) | **PATCH** /api/v2/constraints/{constraint_name} | 



## create_default_constraint

> crate::models::Constraint create_default_constraint(body)


Creates default constraint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConstraintBody**](ConstraintBody.md) |  | [required] |

### Return type

[**crate::models::Constraint**](Constraint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_constraint

> crate::models::Constraint get_default_constraint(constraint_name)


Gets an specified default constraint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**constraint_name** | **String** |  | [required] |

### Return type

[**crate::models::Constraint**](Constraint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_default_constraint

> Vec<crate::models::Constraint> list_default_constraint()
List default constraint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Constraint>**](Constraint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_default_constraint

> crate::models::Constraint patch_default_constraint(constraint_name, patch)


Patch a specified default constraint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**constraint_name** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::Constraint**](Constraint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

