# \GkeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_gke_cluster_disk_types**](GkeApi.md#list_gke_cluster_disk_types) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/gke/disktypes | Gets GKE cluster machine disk types.
[**list_gke_cluster_images**](GkeApi.md#list_gke_cluster_images) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/gke/images | Gets GKE cluster images.
[**list_gke_cluster_sizes**](GkeApi.md#list_gke_cluster_sizes) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/gke/sizes | Gets GKE cluster machine sizes.
[**list_gke_cluster_zones**](GkeApi.md#list_gke_cluster_zones) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/gke/zones | Gets GKE cluster zones.
[**list_gke_disk_types**](GkeApi.md#list_gke_disk_types) | **GET** /api/v2/providers/gke/disktypes | Gets GKE machine disk types.
[**list_gke_images**](GkeApi.md#list_gke_images) | **GET** /api/v2/providers/gke/images | 
[**list_gke_versions**](GkeApi.md#list_gke_versions) | **GET** /api/v2/providers/gke/versions | 
[**list_gke_zones**](GkeApi.md#list_gke_zones) | **GET** /api/v2/providers/gke/zones | 
[**list_gkevm_sizes**](GkeApi.md#list_gkevm_sizes) | **GET** /api/v2/providers/gke/vmsizes | 
[**validate_gke_credentials**](GkeApi.md#validate_gke_credentials) | **GET** /api/v2/providers/gke/validatecredentials | 



## list_gke_cluster_disk_types

> Vec<crate::models::GcpDiskType> list_gke_cluster_disk_types(project_id, cluster_id)
Gets GKE cluster machine disk types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::GcpDiskType>**](GCPDiskType.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gke_cluster_images

> Vec<crate::models::GkeImage> list_gke_cluster_images(project_id, cluster_id)
Gets GKE cluster images.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::GkeImage>**](GKEImage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gke_cluster_sizes

> Vec<crate::models::GcpMachineSize> list_gke_cluster_sizes(project_id, cluster_id)
Gets GKE cluster machine sizes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::GcpMachineSize>**](GCPMachineSize.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gke_cluster_zones

> Vec<crate::models::GkeZone> list_gke_cluster_zones(project_id, cluster_id)
Gets GKE cluster zones.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::GkeZone>**](GKEZone.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gke_disk_types

> Vec<crate::models::GkeDiskType> list_gke_disk_types()
Gets GKE machine disk types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GkeDiskType>**](GKEDiskType.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gke_images

> Vec<crate::models::GkeImage> list_gke_images(service_account, credential, zone)


Lists GKE image types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | Option<**String**> | The plain GCP service account |  |
**credential** | Option<**String**> | The credential name used in the preset for the GCP provider |  |
**zone** | Option<**String**> | The zone name |  |

### Return type

[**Vec<crate::models::GkeImage>**](GKEImage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gke_versions

> Vec<crate::models::MasterVersion> list_gke_versions(service_account, credential, zone, mode, release_channel)


Lists GKE versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | Option<**String**> | The plain GCP service account |  |
**credential** | Option<**String**> | The credential name used in the preset for the GCP provider |  |
**zone** | Option<**String**> | The zone name |  |
**mode** | Option<**String**> | The Mode is how you want GKE Control plane version to be managed. Manual: Manually manage the version upgrades. Auto: automatically manage the cluster's control plane version. |  |
**release_channel** | Option<**String**> | The ReleaseChannel |  |

### Return type

[**Vec<crate::models::MasterVersion>**](MasterVersion.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gke_zones

> Vec<crate::models::GkeZone> list_gke_zones()


Lists GKE zones

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GkeZone>**](GKEZone.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gkevm_sizes

> Vec<crate::models::GcpMachineSize> list_gkevm_sizes(service_account, credential, zone)


Lists GKE vmsizes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | Option<**String**> | The plain GCP service account |  |
**credential** | Option<**String**> | The credential name used in the preset for the GCP provider |  |
**zone** | Option<**String**> | The zone name |  |

### Return type

[**Vec<crate::models::GcpMachineSize>**](GCPMachineSize.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_gke_credentials

> validate_gke_credentials(service_account, credential)


Validates GKE credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | Option<**String**> | The plain GCP service account |  |
**credential** | Option<**String**> | The credential name used in the preset for the GCP provider |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

