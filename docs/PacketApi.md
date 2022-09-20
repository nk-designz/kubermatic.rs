# \PacketApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_packet_sizes**](PacketApi.md#list_packet_sizes) | **GET** /api/v1/providers/packet/sizes | 
[**list_packet_sizes_no_credentials**](PacketApi.md#list_packet_sizes_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/packet/sizes | 
[**list_packet_sizes_no_credentials_v2**](PacketApi.md#list_packet_sizes_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/packet/sizes | 



## list_packet_sizes

> Vec<crate::models::PacketSizeList> list_packet_sizes(api_key, project_id, credential)


Lists sizes from packet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PacketSizeList>**](PacketSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_packet_sizes_no_credentials

> Vec<crate::models::PacketSizeList> list_packet_sizes_no_credentials(project_id, dc, cluster_id)


Lists sizes from packet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PacketSizeList>**](PacketSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_packet_sizes_no_credentials_v2

> Vec<crate::models::PacketSizeList> list_packet_sizes_no_credentials_v2(project_id, cluster_id)


Lists sizes from packet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PacketSizeList>**](PacketSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

