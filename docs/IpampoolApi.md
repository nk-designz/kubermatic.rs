# \IpampoolApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ipam_pool**](IpampoolApi.md#create_ipam_pool) | **POST** /api/v2/seeds/{seed_name}/ipampools | Creates a IPAM pool.
[**delete_ipam_pool**](IpampoolApi.md#delete_ipam_pool) | **DELETE** /api/v2/seeds/{seed_name}/ipampools/{ipampool_name} | Removes an existing IPAM pool.
[**get_ipam_pool**](IpampoolApi.md#get_ipam_pool) | **GET** /api/v2/seeds/{seed_name}/ipampools/{ipampool_name} | Gets a specific IPAM pool.
[**list_ipam_pools**](IpampoolApi.md#list_ipam_pools) | **GET** /api/v2/seeds/{seed_name}/ipampools | Lists IPAM pools.
[**patch_ipam_pool**](IpampoolApi.md#patch_ipam_pool) | **PATCH** /api/v2/seeds/{seed_name}/ipampools/{ipampool_name} | Patches a IPAM pool.



## create_ipam_pool

> create_ipam_pool(seed_name, body)
Creates a IPAM pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**body** | [**IpamPool**](IpamPool.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ipam_pool

> delete_ipam_pool(seed_name, ipampool_name)
Removes an existing IPAM pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**ipampool_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ipam_pool

> crate::models::IpamPool get_ipam_pool(seed_name, ipampool_name)
Gets a specific IPAM pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**ipampool_name** | **String** |  | [required] |

### Return type

[**crate::models::IpamPool**](IPAMPool.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ipam_pools

> Vec<crate::models::IpamPool> list_ipam_pools(seed_name)
Lists IPAM pools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |

### Return type

[**Vec<crate::models::IpamPool>**](IPAMPool.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_ipam_pool

> patch_ipam_pool(seed_name, ipampool_name, body)
Patches a IPAM pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**ipampool_name** | **String** |  | [required] |
**body** | [**IpamPool**](IpamPool.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

