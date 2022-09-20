# \AlibabaApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_alibaba_instance_types**](AlibabaApi.md#list_alibaba_instance_types) | **GET** /api/v1/providers/alibaba/instancetypes | Lists available Alibaba instance types.
[**list_alibaba_instance_types_no_credentials**](AlibabaApi.md#list_alibaba_instance_types_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/alibaba/instancetypes | 
[**list_alibaba_instance_types_no_credentials_v2**](AlibabaApi.md#list_alibaba_instance_types_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/alibaba/instancetypes | 
[**list_alibaba_v_switches**](AlibabaApi.md#list_alibaba_v_switches) | **GET** /api/v1/providers/alibaba/vswitches | Lists available Alibaba vSwitches.
[**list_alibaba_v_switches_no_credentials_v2**](AlibabaApi.md#list_alibaba_v_switches_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/alibaba/vswitches | 
[**list_alibaba_zones**](AlibabaApi.md#list_alibaba_zones) | **GET** /api/v1/providers/alibaba/zones | Lists available Alibaba zones.
[**list_alibaba_zones_no_credentials**](AlibabaApi.md#list_alibaba_zones_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/alibaba/zones | 
[**list_alibaba_zones_no_credentials_v2**](AlibabaApi.md#list_alibaba_zones_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/alibaba/zones | 



## list_alibaba_instance_types

> crate::models::AlibabaInstanceTypeList list_alibaba_instance_types(access_key_id, access_key_secret, credential, region)
Lists available Alibaba instance types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> |  |  |
**access_key_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |

### Return type

[**crate::models::AlibabaInstanceTypeList**](AlibabaInstanceTypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alibaba_instance_types_no_credentials

> crate::models::AlibabaInstanceTypeList list_alibaba_instance_types_no_credentials(project_id, dc, cluster_id, region)


Lists available Alibaba Instance Types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**region** | Option<**String**> |  |  |

### Return type

[**crate::models::AlibabaInstanceTypeList**](AlibabaInstanceTypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alibaba_instance_types_no_credentials_v2

> crate::models::AlibabaInstanceTypeList list_alibaba_instance_types_no_credentials_v2(project_id, cluster_id, region)


Lists available Alibaba Instance Types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**region** | Option<**String**> |  |  |

### Return type

[**crate::models::AlibabaInstanceTypeList**](AlibabaInstanceTypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alibaba_v_switches

> crate::models::AlibabaVSwitchList list_alibaba_v_switches()
Lists available Alibaba vSwitches.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AlibabaVSwitchList**](AlibabaVSwitchList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alibaba_v_switches_no_credentials_v2

> crate::models::AlibabaVSwitchList list_alibaba_v_switches_no_credentials_v2(project_id, cluster_id, region)


Lists available Alibaba vSwitches

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**region** | Option<**String**> |  |  |

### Return type

[**crate::models::AlibabaVSwitchList**](AlibabaVSwitchList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alibaba_zones

> crate::models::AlibabaZoneList list_alibaba_zones(access_key_id, access_key_secret, credential, region)
Lists available Alibaba zones.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> |  |  |
**access_key_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |

### Return type

[**crate::models::AlibabaZoneList**](AlibabaZoneList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alibaba_zones_no_credentials

> crate::models::AlibabaZoneList list_alibaba_zones_no_credentials(project_id, dc, cluster_id, region)


Lists available Alibaba Instance Types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**region** | Option<**String**> |  |  |

### Return type

[**crate::models::AlibabaZoneList**](AlibabaZoneList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alibaba_zones_no_credentials_v2

> crate::models::AlibabaZoneList list_alibaba_zones_no_credentials_v2(project_id, cluster_id, region)


Lists available Alibaba Instance Types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**region** | Option<**String**> |  |  |

### Return type

[**crate::models::AlibabaZoneList**](AlibabaZoneList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

