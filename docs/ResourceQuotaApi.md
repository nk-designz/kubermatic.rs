# \ResourceQuotaApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resource_quota**](ResourceQuotaApi.md#create_resource_quota) | **POST** /api/v2/quotas | Creates a new Resource Quota.
[**delete_resource_quota**](ResourceQuotaApi.md#delete_resource_quota) | **DELETE** /api/v2/quotas/{quota_name} | Removes an existing Resource Quota.
[**get_resource_quota**](ResourceQuotaApi.md#get_resource_quota) | **GET** /api/v2/quotas/{quota_name} | Gets a specific Resource Quota.
[**list_resource_quotas**](ResourceQuotaApi.md#list_resource_quotas) | **GET** /api/v2/quotas | Gets a Resource Quota list.
[**put_resource_quota**](ResourceQuotaApi.md#put_resource_quota) | **PUT** /api/v2/quotas/{quota_name} | Updates an existing Resource Quota.



## create_resource_quota

> create_resource_quota(body)
Creates a new Resource Quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V2QuotasBody**](V2QuotasBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource_quota

> delete_resource_quota(quota_name)
Removes an existing Resource Quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_quota

> crate::models::ResourceQuota get_resource_quota(quota_name)
Gets a specific Resource Quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_name** | **String** |  | [required] |

### Return type

[**crate::models::ResourceQuota**](ResourceQuota.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resource_quotas

> Vec<crate::models::ResourceQuota> list_resource_quotas(subject_name, subject_kind)
Gets a Resource Quota list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_name** | Option<**String**> |  |  |
**subject_kind** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ResourceQuota>**](ResourceQuota.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_resource_quota

> put_resource_quota(quota_name, body)
Updates an existing Resource Quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_name** | **String** |  | [required] |
**body** | [**Quota**](Quota.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

