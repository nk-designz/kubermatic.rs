# \KubevirtApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_kube_virt_vmi_presets**](KubevirtApi.md#list_kube_virt_vmi_presets) | **GET** /api/v2/providers/kubevirt/vmflavors | Lists available KubeVirt VirtualMachineInstancePreset.
[**list_kube_virt_vmi_presets_no_credentials**](KubevirtApi.md#list_kube_virt_vmi_presets_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/kubevirt/vmflavors | 
[**list_kubevirt_storage_classes**](KubevirtApi.md#list_kubevirt_storage_classes) | **GET** /api/v2/providers/kubevirt/storageclasses | Lists available K8s StorageClasses in the Kubevirt cluster.
[**list_kubevirt_storage_classes_no_credentials**](KubevirtApi.md#list_kubevirt_storage_classes_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/kubevirt/storageclasses | 



## list_kube_virt_vmi_presets

> Vec<crate::models::VirtualMachineInstancePreset> list_kube_virt_vmi_presets(kubeconfig, credential)
Lists available KubeVirt VirtualMachineInstancePreset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kubeconfig** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::VirtualMachineInstancePreset>**](VirtualMachineInstancePreset.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_kube_virt_vmi_presets_no_credentials

> Vec<crate::models::VirtualMachineInstancePreset> list_kube_virt_vmi_presets_no_credentials(project_id, cluster_id)


Lists available VirtualMachineInstancePreset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VirtualMachineInstancePreset>**](VirtualMachineInstancePreset.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_kubevirt_storage_classes

> Vec<crate::models::StorageClass> list_kubevirt_storage_classes(kubeconfig, credential)
Lists available K8s StorageClasses in the Kubevirt cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kubeconfig** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::StorageClass>**](StorageClass.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_kubevirt_storage_classes_no_credentials

> Vec<crate::models::StorageClass> list_kubevirt_storage_classes_no_credentials(project_id, cluster_id)


List Storage Classes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::StorageClass>**](StorageClass.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

