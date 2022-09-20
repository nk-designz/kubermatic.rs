# \EtcdrestoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_etcd_restore**](EtcdrestoreApi.md#create_etcd_restore) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdrestores | 
[**delete_etcd_restore**](EtcdrestoreApi.md#delete_etcd_restore) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdrestores/{er_name} | 
[**get_etcd_restore**](EtcdrestoreApi.md#get_etcd_restore) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdrestores/{er_name} | 
[**list_etcd_restore**](EtcdrestoreApi.md#list_etcd_restore) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdrestores | 
[**list_project_etcd_restore**](EtcdrestoreApi.md#list_project_etcd_restore) | **GET** /api/v2/projects/{project_id}/etcdrestores | 



## create_etcd_restore

> crate::models::EtcdBackupConfig create_etcd_restore(project_id, cluster_id, body)


Creates a etcd backup restore for a given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**ErBody**](ErBody.md)> |  |  |

### Return type

[**crate::models::EtcdBackupConfig**](EtcdBackupConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_etcd_restore

> delete_etcd_restore(project_id, cluster_id, er_name)


Deletes a etcd restore config for a given cluster based on its name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**er_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etcd_restore

> crate::models::EtcdRestore get_etcd_restore(project_id, cluster_id, er_name)


Gets a etcd backup restore for a given cluster based on its name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**er_name** | **String** |  | [required] |

### Return type

[**crate::models::EtcdRestore**](EtcdRestore.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_etcd_restore

> Vec<crate::models::EtcdRestore> list_etcd_restore(project_id, cluster_id)


List etcd backup restores for a given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::EtcdRestore>**](EtcdRestore.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_etcd_restore

> Vec<crate::models::EtcdRestore> list_project_etcd_restore(project_id)


List etcd backup restores for a given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::EtcdRestore>**](EtcdRestore.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

