# \DatacenterApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dc**](DatacenterApi.md#create_dc) | **POST** /api/v1/seed/{seed_name}/dc | Create the datacenter for a specified seed.
[**delete_dc**](DatacenterApi.md#delete_dc) | **DELETE** /api/v1/seed/{seed_name}/dc/{dc} | Delete the datacenter.
[**get_datacenter**](DatacenterApi.md#get_datacenter) | **GET** /api/v1/dc/{dc} | 
[**get_dc_for_provider**](DatacenterApi.md#get_dc_for_provider) | **GET** /api/v1/providers/{provider_name}/dc/{dc} | Get the datacenter for the specified provider.
[**get_dc_for_seed**](DatacenterApi.md#get_dc_for_seed) | **GET** /api/v1/seed/{seed_name}/dc/{dc} | Returns the specified datacenter for the specified seed.
[**list_datacenters**](DatacenterApi.md#list_datacenters) | **GET** /api/v1/dc | 
[**list_dc_for_provider**](DatacenterApi.md#list_dc_for_provider) | **GET** /api/v1/providers/{provider_name}/dc | Returns all datacenters for the specified provider.
[**list_dc_for_seed**](DatacenterApi.md#list_dc_for_seed) | **GET** /api/v1/seed/{seed_name}/dc | Returns all datacenters for the specified seed.
[**patch_dc**](DatacenterApi.md#patch_dc) | **PATCH** /api/v1/seed/{seed_name}/dc/{dc} | Patch the datacenter.
[**update_dc**](DatacenterApi.md#update_dc) | **PUT** /api/v1/seed/{seed_name}/dc/{dc} | Update the datacenter. The datacenter spec will be overwritten with the one provided in the request.



## create_dc

> crate::models::Datacenter create_dc(seed_name, body)
Create the datacenter for a specified seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**body** | Option<[**InlineObject5**](InlineObject5.md)> |  |  |

### Return type

[**crate::models::Datacenter**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dc

> delete_dc(seed_name, dc)
Delete the datacenter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**dc** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_datacenter

> crate::models::Datacenter get_datacenter(dc)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** |  | [required] |

### Return type

[**crate::models::Datacenter**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dc_for_provider

> crate::models::Datacenter get_dc_for_provider(provider_name, dc)
Get the datacenter for the specified provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**dc** | **String** |  | [required] |

### Return type

[**crate::models::Datacenter**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dc_for_seed

> crate::models::Datacenter get_dc_for_seed(seed_name, dc)
Returns the specified datacenter for the specified seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**dc** | **String** |  | [required] |

### Return type

[**crate::models::Datacenter**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_datacenters

> Vec<crate::models::Datacenter> list_datacenters()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Datacenter>**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dc_for_provider

> Vec<crate::models::Datacenter> list_dc_for_provider(provider_name)
Returns all datacenters for the specified provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Datacenter>**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dc_for_seed

> Vec<crate::models::Datacenter> list_dc_for_seed(seed_name)
Returns all datacenters for the specified seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Datacenter>**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_dc

> crate::models::Datacenter patch_dc(dc, seed_name, patch)
Patch the datacenter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** |  | [required] |
**seed_name** | **String** |  | [required] |
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::Datacenter**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dc

> crate::models::Datacenter update_dc(seed_name, dc, body)
Update the datacenter. The datacenter spec will be overwritten with the one provided in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**body** | Option<[**InlineObject6**](InlineObject6.md)> |  |  |

### Return type

[**crate::models::Datacenter**](Datacenter.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

