# \GcpApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_gcp_disk_types**](GcpApi.md#list_gcp_disk_types) | **GET** /api/v1/providers/gcp/disktypes | 
[**list_gcp_disk_types_no_credentials**](GcpApi.md#list_gcp_disk_types_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/gcp/disktypes | 
[**list_gcp_disk_types_no_credentials_v2**](GcpApi.md#list_gcp_disk_types_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/gcp/disktypes | 
[**list_gcp_networks**](GcpApi.md#list_gcp_networks) | **GET** /api/v1/providers/gcp/networks | 
[**list_gcp_networks_no_credentials**](GcpApi.md#list_gcp_networks_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/gcp/networks | 
[**list_gcp_networks_no_credentials_v2**](GcpApi.md#list_gcp_networks_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/gcp/networks | 
[**list_gcp_sizes**](GcpApi.md#list_gcp_sizes) | **GET** /api/v1/providers/gcp/sizes | 
[**list_gcp_sizes_no_credentials**](GcpApi.md#list_gcp_sizes_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/gcp/sizes | 
[**list_gcp_sizes_no_credentials_v2**](GcpApi.md#list_gcp_sizes_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/gcp/sizes | 
[**list_gcp_subnetworks**](GcpApi.md#list_gcp_subnetworks) | **GET** /api/v1/providers/gcp/{dc}/subnetworks | 
[**list_gcp_subnetworks_no_credentials**](GcpApi.md#list_gcp_subnetworks_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/gcp/subnetworks | 
[**list_gcp_subnetworks_no_credentials_v2**](GcpApi.md#list_gcp_subnetworks_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/gcp/subnetworks | 
[**list_gcp_zones**](GcpApi.md#list_gcp_zones) | **GET** /api/v1/providers/gcp/{dc}/zones | 
[**list_gcp_zones_no_credentials**](GcpApi.md#list_gcp_zones_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/gcp/zones | 
[**list_gcp_zones_no_credentials_v2**](GcpApi.md#list_gcp_zones_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/gcp/zones | 



## list_gcp_disk_types

> crate::models::GcpDiskTypeList list_gcp_disk_types(service_account, credential, zone)


Lists disk types from GCP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**zone** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpDiskTypeList**](GCPDiskTypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_disk_types_no_credentials

> crate::models::GcpDiskTypeList list_gcp_disk_types_no_credentials(project_id, dc, cluster_id, zone)


Lists disk types from GCP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**zone** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpDiskTypeList**](GCPDiskTypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_disk_types_no_credentials_v2

> crate::models::GcpDiskTypeList list_gcp_disk_types_no_credentials_v2(project_id, cluster_id, zone)


Lists disk types from GCP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**zone** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpDiskTypeList**](GCPDiskTypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_networks

> crate::models::GcpNetworkList list_gcp_networks()


Lists networks from GCP

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GcpNetworkList**](GCPNetworkList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_networks_no_credentials

> crate::models::GcpNetworkList list_gcp_networks_no_credentials(project_id, dc, cluster_id)


Lists available GCP networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::GcpNetworkList**](GCPNetworkList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_networks_no_credentials_v2

> crate::models::GcpNetworkList list_gcp_networks_no_credentials_v2(project_id, cluster_id)


Lists available GCP networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::GcpNetworkList**](GCPNetworkList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_sizes

> crate::models::GcpMachineSizeList list_gcp_sizes(service_account, credential, zone)


Lists machine sizes from GCP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**zone** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpMachineSizeList**](GCPMachineSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_sizes_no_credentials

> crate::models::GcpMachineSizeList list_gcp_sizes_no_credentials(project_id, dc, cluster_id, zone)


Lists machine sizes from GCP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**zone** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpMachineSizeList**](GCPMachineSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_sizes_no_credentials_v2

> crate::models::GcpMachineSizeList list_gcp_sizes_no_credentials_v2(project_id, cluster_id, zone)


Lists machine sizes from GCP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**zone** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpMachineSizeList**](GCPMachineSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_subnetworks

> crate::models::GcpSubnetworkList list_gcp_subnetworks(dc, service_account, credential, network)


Lists subnetworks from GCP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** |  | [required] |
**service_account** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**network** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpSubnetworkList**](GCPSubnetworkList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_subnetworks_no_credentials

> crate::models::GcpSubnetworkList list_gcp_subnetworks_no_credentials(project_id, dc, cluster_id, network)


Lists available GCP subnetworks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**network** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpSubnetworkList**](GCPSubnetworkList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_subnetworks_no_credentials_v2

> crate::models::GcpSubnetworkList list_gcp_subnetworks_no_credentials_v2(project_id, cluster_id, network)


Lists available GCP subnetworks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**network** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpSubnetworkList**](GCPSubnetworkList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_zones

> crate::models::GcpZoneList list_gcp_zones(dc, service_account, credential)


Lists available GCP zones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** |  | [required] |
**service_account** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**crate::models::GcpZoneList**](GCPZoneList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_zones_no_credentials

> crate::models::GcpZoneList list_gcp_zones_no_credentials(project_id, dc, cluster_id)


Lists available GCP zones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::GcpZoneList**](GCPZoneList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_zones_no_credentials_v2

> crate::models::GcpZoneList list_gcp_zones_no_credentials_v2(project_id, cluster_id)


Lists available GCP zones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::GcpZoneList**](GCPZoneList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

