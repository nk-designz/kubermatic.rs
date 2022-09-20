# \MlaadminsettingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mla_admin_setting**](MlaadminsettingApi.md#create_mla_admin_setting) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/mlaadminsetting | 
[**delete_mla_admin_setting**](MlaadminsettingApi.md#delete_mla_admin_setting) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/mlaadminsetting | Deletes the MLA admin setting that belongs to the cluster.
[**get_mla_admin_setting**](MlaadminsettingApi.md#get_mla_admin_setting) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/mlaadminsetting | Gets MLA Admin settings for the given cluster.
[**update_mla_admin_setting**](MlaadminsettingApi.md#update_mla_admin_setting) | **PUT** /api/v2/projects/{project_id}/clusters/{cluster_id}/mlaadminsetting | Updates the MLA admin setting for the given cluster.



## create_mla_admin_setting

> crate::models::MlaAdminSetting create_mla_admin_setting(project_id, cluster_id, body)


Creates MLA admin setting that will belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | [**MlaAdminSetting**](MlaAdminSetting.md) |  | [required] |

### Return type

[**crate::models::MlaAdminSetting**](MLAAdminSetting.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mla_admin_setting

> delete_mla_admin_setting(project_id, cluster_id)
Deletes the MLA admin setting that belongs to the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mla_admin_setting

> crate::models::MlaAdminSetting get_mla_admin_setting(project_id, cluster_id)
Gets MLA Admin settings for the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::MlaAdminSetting**](MLAAdminSetting.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_mla_admin_setting

> crate::models::MlaAdminSetting update_mla_admin_setting(project_id, cluster_id, body)
Updates the MLA admin setting for the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | [**MlaAdminSetting**](MlaAdminSetting.md) |  | [required] |

### Return type

[**crate::models::MlaAdminSetting**](MLAAdminSetting.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

