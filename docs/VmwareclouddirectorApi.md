# \VmwareclouddirectorApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_v_mware_cloud_director_catalogs**](VmwareclouddirectorApi.md#list_v_mware_cloud_director_catalogs) | **GET** /api/v2/providers/vmwareclouddirector/{dc}/catalogs | 
[**list_v_mware_cloud_director_catalogs_no_credentials**](VmwareclouddirectorApi.md#list_v_mware_cloud_director_catalogs_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/vmwareclouddirector/catalogs | 
[**list_v_mware_cloud_director_networks**](VmwareclouddirectorApi.md#list_v_mware_cloud_director_networks) | **GET** /api/v2/providers/vmwareclouddirector/{dc}/networks | 
[**list_v_mware_cloud_director_networks_no_credentials**](VmwareclouddirectorApi.md#list_v_mware_cloud_director_networks_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/vmwareclouddirector/networks | 
[**list_v_mware_cloud_director_storage_profiles**](VmwareclouddirectorApi.md#list_v_mware_cloud_director_storage_profiles) | **GET** /api/v2/providers/vmwareclouddirector/{dc}/storageprofiles | 
[**list_v_mware_cloud_director_storage_profiles_no_credentials**](VmwareclouddirectorApi.md#list_v_mware_cloud_director_storage_profiles_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/vmwareclouddirector/storageprofiles | 
[**list_v_mware_cloud_director_templates**](VmwareclouddirectorApi.md#list_v_mware_cloud_director_templates) | **GET** /api/v2/providers/vmwareclouddirector/{dc}/templates/{catalog_name} | 
[**list_v_mware_cloud_director_templates_no_credentials**](VmwareclouddirectorApi.md#list_v_mware_cloud_director_templates_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/vmwareclouddirector/templates/{catalog_name} | 



## list_v_mware_cloud_director_catalogs

> Vec<crate::models::VMwareCloudDirectorCatalog> list_v_mware_cloud_director_catalogs(dc, username, password, organization, VDC, credential)


List VMware Cloud Director Catalogs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**organization** | Option<**String**> |  |  |
**VDC** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VMwareCloudDirectorCatalog>**](VMwareCloudDirectorCatalog.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_mware_cloud_director_catalogs_no_credentials

> Vec<crate::models::VMwareCloudDirectorCatalog> list_v_mware_cloud_director_catalogs_no_credentials(project_id, cluster_id)


List VMware Cloud Director Catalogs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VMwareCloudDirectorCatalog>**](VMwareCloudDirectorCatalog.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_mware_cloud_director_networks

> Vec<crate::models::VMwareCloudDirectorNetwork> list_v_mware_cloud_director_networks(dc, username, password, organization, VDC, credential)


List VMware Cloud Director OVDC Networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**organization** | Option<**String**> |  |  |
**VDC** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VMwareCloudDirectorNetwork>**](VMwareCloudDirectorNetwork.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_mware_cloud_director_networks_no_credentials

> Vec<crate::models::VMwareCloudDirectorNetwork> list_v_mware_cloud_director_networks_no_credentials(project_id, cluster_id)


List VMware Cloud Director OVDC Networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VMwareCloudDirectorNetwork>**](VMwareCloudDirectorNetwork.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_mware_cloud_director_storage_profiles

> Vec<crate::models::VMwareCloudDirectorStorageProfile> list_v_mware_cloud_director_storage_profiles(dc, username, password, organization, VDC, credential)


List VMware Cloud Director Storage Profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**organization** | Option<**String**> |  |  |
**VDC** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VMwareCloudDirectorStorageProfile>**](VMwareCloudDirectorStorageProfile.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_mware_cloud_director_storage_profiles_no_credentials

> Vec<crate::models::VMwareCloudDirectorStorageProfile> list_v_mware_cloud_director_storage_profiles_no_credentials(project_id, cluster_id)


List VMware Cloud Director Storage Profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VMwareCloudDirectorStorageProfile>**](VMwareCloudDirectorStorageProfile.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_mware_cloud_director_templates

> Vec<crate::models::VMwareCloudDirectorTemplate> list_v_mware_cloud_director_templates(dc, catalog_name, username, password, organization, VDC, credential)


List VMware Cloud Director Templates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**catalog_name** | **String** | Catalog name to fetch the templates from | [required] |
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**organization** | Option<**String**> |  |  |
**VDC** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VMwareCloudDirectorTemplate>**](VMwareCloudDirectorTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_mware_cloud_director_templates_no_credentials

> Vec<crate::models::VMwareCloudDirectorTemplate> list_v_mware_cloud_director_templates_no_credentials(project_id, cluster_id, catalog_name)


List VMware Cloud Director Templates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**catalog_name** | **String** | Catalog name to fetch the templates from | [required] |

### Return type

[**Vec<crate::models::VMwareCloudDirectorTemplate>**](VMwareCloudDirectorTemplate.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

