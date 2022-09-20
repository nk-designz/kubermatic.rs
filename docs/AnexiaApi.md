# \AnexiaApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_anexia_templates**](AnexiaApi.md#list_anexia_templates) | **GET** /api/v1/providers/anexia/templates | 
[**list_anexia_templates_no_credentials_v2**](AnexiaApi.md#list_anexia_templates_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/anexia/templates | 
[**list_anexia_vlans**](AnexiaApi.md#list_anexia_vlans) | **GET** /api/v1/providers/anexia/vlans | 
[**list_anexia_vlans_no_credentials_v2**](AnexiaApi.md#list_anexia_vlans_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/anexia/vlans | 



## list_anexia_templates

> Vec<crate::models::AnexiaTemplate> list_anexia_templates(token, credential, location)


Lists templates from anexia

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AnexiaTemplate>**](AnexiaTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_anexia_templates_no_credentials_v2

> Vec<crate::models::AnexiaTemplate> list_anexia_templates_no_credentials_v2(project_id, cluster_id)


Lists templates from Anexia

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::AnexiaTemplate>**](AnexiaTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_anexia_vlans

> Vec<crate::models::AnexiaVlan> list_anexia_vlans(token, credential)


Lists vlans from anexia

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AnexiaVlan>**](AnexiaVlan.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_anexia_vlans_no_credentials_v2

> Vec<crate::models::AnexiaVlan> list_anexia_vlans_no_credentials_v2(project_id, cluster_id)


Lists vlans from Anexia

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::AnexiaVlan>**](AnexiaVlan.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

