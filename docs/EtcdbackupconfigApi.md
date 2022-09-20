# \EtcdbackupconfigApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_etcd_backup_config**](EtcdbackupconfigApi.md#create_etcd_backup_config) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdbackupconfigs | 
[**delete_etcd_backup_config**](EtcdbackupconfigApi.md#delete_etcd_backup_config) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdbackupconfigs/{ebc_id} | 
[**get_etcd_backup_config**](EtcdbackupconfigApi.md#get_etcd_backup_config) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdbackupconfigs/{ebc_id} | 
[**list_etcd_backup_config**](EtcdbackupconfigApi.md#list_etcd_backup_config) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdbackupconfigs | 
[**list_project_etcd_backup_config**](EtcdbackupconfigApi.md#list_project_etcd_backup_config) | **GET** /api/v2/projects/{project_id}/etcdbackupconfigs | 
[**patch_etcd_backup_config**](EtcdbackupconfigApi.md#patch_etcd_backup_config) | **PATCH** /api/v2/projects/{project_id}/clusters/{cluster_id}/etcdbackupconfigs/{ebc_id} | 



## create_etcd_backup_config

> crate::models::EtcdBackupConfig create_etcd_backup_config(project_id, cluster_id, body)


Creates a etcd backup config that will belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**EbcBody**](EbcBody.md)> |  |  |

### Return type

[**crate::models::EtcdBackupConfig**](EtcdBackupConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_etcd_backup_config

> delete_etcd_backup_config(project_id, cluster_id, ebc_id)


Deletes a etcd backup config for a given cluster based on its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**ebc_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etcd_backup_config

> crate::models::EtcdBackupConfig get_etcd_backup_config(project_id, cluster_id, ebc_id)


Gets a etcd backup config for a given cluster based on its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**ebc_id** | **String** |  | [required] |

### Return type

[**crate::models::EtcdBackupConfig**](EtcdBackupConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_etcd_backup_config

> Vec<crate::models::EtcdBackupConfig> list_etcd_backup_config(project_id, cluster_id)


List etcd backup configs for a given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::EtcdBackupConfig>**](EtcdBackupConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_etcd_backup_config

> Vec<crate::models::EtcdBackupConfig> list_project_etcd_backup_config(project_id, _type)


List etcd backup configs for a given project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EtcdBackupConfig>**](EtcdBackupConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_etcd_backup_config

> crate::models::EtcdBackupConfig patch_etcd_backup_config(project_id, cluster_id, ebc_id, body)


Patches a etcd backup config for a given cluster based on its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**ebc_id** | **String** |  | [required] |
**body** | [**EtcdBackupConfigSpec**](EtcdBackupConfigSpec.md) |  | [required] |

### Return type

[**crate::models::EtcdBackupConfig**](EtcdBackupConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

