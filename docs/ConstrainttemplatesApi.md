# \ConstrainttemplatesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_constraint_template**](ConstrainttemplatesApi.md#create_constraint_template) | **POST** /api/v2/constrainttemplates | 
[**delete_constraint_template**](ConstrainttemplatesApi.md#delete_constraint_template) | **DELETE** /api/v2/constrainttemplates/{ct_name} | 
[**get_constraint_template**](ConstrainttemplatesApi.md#get_constraint_template) | **GET** /api/v2/constrainttemplates/{ct_name} | 
[**list_constraint_templates**](ConstrainttemplatesApi.md#list_constraint_templates) | **GET** /api/v2/constrainttemplates | List constraint templates.
[**patch_constraint_template**](ConstrainttemplatesApi.md#patch_constraint_template) | **PATCH** /api/v2/constrainttemplates/{ct_name} | 



## create_constraint_template

> crate::models::ConstraintTemplate create_constraint_template(body)


Create constraint template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CtBody**](CtBody.md)> |  |  |

### Return type

[**crate::models::ConstraintTemplate**](ConstraintTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_constraint_template

> delete_constraint_template(ct_name)


Deletes the specified cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ct_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_constraint_template

> crate::models::ConstraintTemplate get_constraint_template(ct_name)


Get constraint templates specified by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ct_name** | **String** |  | [required] |

### Return type

[**crate::models::ConstraintTemplate**](ConstraintTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_constraint_templates

> Vec<crate::models::ConstraintTemplate> list_constraint_templates()
List constraint templates.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ConstraintTemplate>**](ConstraintTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_constraint_template

> crate::models::ConstraintTemplate patch_constraint_template(ct_name, patch)


Patch a specified constraint template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ct_name** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::ConstraintTemplate**](ConstraintTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

