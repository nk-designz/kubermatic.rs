# \DigitaloceanApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_digitalocean_sizes**](DigitaloceanApi.md#list_digitalocean_sizes) | **GET** /api/v1/providers/digitalocean/sizes | 
[**list_digitalocean_sizes_no_credentials**](DigitaloceanApi.md#list_digitalocean_sizes_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/digitalocean/sizes | 
[**list_digitalocean_sizes_no_credentials_v2**](DigitaloceanApi.md#list_digitalocean_sizes_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/digitalocean/sizes | 



## list_digitalocean_sizes

> crate::models::DigitaloceanSizeList list_digitalocean_sizes(do_token, credential)


Lists sizes from digitalocean

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**do_token** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**crate::models::DigitaloceanSizeList**](DigitaloceanSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_digitalocean_sizes_no_credentials

> crate::models::DigitaloceanSizeList list_digitalocean_sizes_no_credentials(project_id, dc, cluster_id)


Lists sizes from digitalocean

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::DigitaloceanSizeList**](DigitaloceanSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_digitalocean_sizes_no_credentials_v2

> crate::models::DigitaloceanSizeList list_digitalocean_sizes_no_credentials_v2(project_id, cluster_id)


Lists sizes from digitalocean

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::DigitaloceanSizeList**](DigitaloceanSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

