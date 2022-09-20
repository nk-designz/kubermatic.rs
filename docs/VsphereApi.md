# \VsphereApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_v_sphere_datastores**](VsphereApi.md#list_v_sphere_datastores) | **GET** /api/v2/providers/vsphere/datastores | 
[**list_v_sphere_folders**](VsphereApi.md#list_v_sphere_folders) | **GET** /api/v1/providers/vsphere/folders | 
[**list_v_sphere_folders_no_credentials**](VsphereApi.md#list_v_sphere_folders_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/vsphere/folders | 
[**list_v_sphere_folders_no_credentials_v2**](VsphereApi.md#list_v_sphere_folders_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/vsphere/folders | 
[**list_v_sphere_networks**](VsphereApi.md#list_v_sphere_networks) | **GET** /api/v1/providers/vsphere/networks | 
[**list_v_sphere_networks_no_credentials**](VsphereApi.md#list_v_sphere_networks_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/vsphere/networks | 
[**list_v_sphere_networks_no_credentials_v2**](VsphereApi.md#list_v_sphere_networks_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/vsphere/networks | 



## list_v_sphere_datastores

> Vec<crate::models::VSphereDatastoreList> list_v_sphere_datastores(username, password, datacenter_name, credential)


Lists datastores from vsphere datacenter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VSphereDatastoreList>**](VSphereDatastoreList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_sphere_folders

> Vec<crate::models::VSphereFolder> list_v_sphere_folders(username, password, datacenter_name, credential)


Lists folders from vsphere datacenter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VSphereFolder>**](VSphereFolder.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_sphere_folders_no_credentials

> Vec<crate::models::VSphereFolder> list_v_sphere_folders_no_credentials(project_id, dc, cluster_id)


Lists folders from vsphere datacenter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VSphereFolder>**](VSphereFolder.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_sphere_folders_no_credentials_v2

> Vec<crate::models::VSphereFolder> list_v_sphere_folders_no_credentials_v2(project_id, cluster_id)


Lists folders from vsphere datacenter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VSphereFolder>**](VSphereFolder.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_sphere_networks

> Vec<crate::models::VSphereNetwork> list_v_sphere_networks(username, password, datacenter_name, credential)


Lists networks from vsphere datacenter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VSphereNetwork>**](VSphereNetwork.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_sphere_networks_no_credentials

> Vec<crate::models::VSphereNetwork> list_v_sphere_networks_no_credentials(project_id, dc, cluster_id)


Lists networks from vsphere datacenter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VSphereNetwork>**](VSphereNetwork.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_v_sphere_networks_no_credentials_v2

> Vec<crate::models::VSphereNetwork> list_v_sphere_networks_no_credentials_v2(project_id, cluster_id)


Lists networks from vsphere datacenter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VSphereNetwork>**](VSphereNetwork.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

