# \AksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_aks_locations**](AksApi.md#list_aks_locations) | **GET** /api/v2/providers/aks/locations | List AKS recommended Locations.
[**list_aks_node_pool_modes**](AksApi.md#list_aks_node_pool_modes) | **GET** /api/v2/providers/aks/modes | Gets the AKS node pool modes.
[**list_aks_node_versions_no_credentials**](AksApi.md#list_aks_node_versions_no_credentials) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/aks/versions | Gets AKS nodepool available versions.
[**list_aks_versions**](AksApi.md#list_aks_versions) | **GET** /api/v2/providers/aks/versions | 
[**list_aksvm_sizes**](AksApi.md#list_aksvm_sizes) | **GET** /api/v2/providers/aks/vmsizes | List AKS available VM sizes in an Azure region.
[**list_aksvm_sizes_no_credentials**](AksApi.md#list_aksvm_sizes_no_credentials) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/aks/vmsizes | Gets AKS available VM sizes in an Azure region.
[**validate_aks_credentials**](AksApi.md#validate_aks_credentials) | **GET** /api/v2/providers/aks/validatecredentials | 



## list_aks_locations

> crate::models::AksLocationList list_aks_locations()
List AKS recommended Locations.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AksLocationList**](AKSLocationList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aks_node_pool_modes

> crate::models::AksNodePoolModes list_aks_node_pool_modes()
Gets the AKS node pool modes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AksNodePoolModes**](AKSNodePoolModes.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aks_node_versions_no_credentials

> Vec<crate::models::MasterVersion> list_aks_node_versions_no_credentials(project_id, cluster_id)
Gets AKS nodepool available versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::MasterVersion>**](MasterVersion.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aks_versions

> Vec<crate::models::MasterVersion> list_aks_versions()


Lists AKS versions

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MasterVersion>**](MasterVersion.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aksvm_sizes

> crate::models::AksvmSizeList list_aksvm_sizes(tenant_id, subscription_id, client_id, client_secret, credential, location)
List AKS available VM sizes in an Azure region.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> |  |  |
**subscription_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**location** | Option<**String**> | Location - Resource location |  |

### Return type

[**crate::models::AksvmSizeList**](AKSVMSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aksvm_sizes_no_credentials

> crate::models::AksvmSizeList list_aksvm_sizes_no_credentials(project_id, cluster_id, location)
Gets AKS available VM sizes in an Azure region.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**location** | Option<**String**> | Location - Resource location |  |

### Return type

[**crate::models::AksvmSizeList**](AKSVMSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_aks_credentials

> validate_aks_credentials(tenant_id, subscription_id, client_id, client_secret, credential)


Validates AKS credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> |  |  |
**subscription_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

