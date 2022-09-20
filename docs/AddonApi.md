# \AddonApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_addon**](AddonApi.md#create_addon) | **POST** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/addons | 
[**create_addon_v2**](AddonApi.md#create_addon_v2) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/addons | 
[**delete_addon**](AddonApi.md#delete_addon) | **DELETE** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/addons/{addon_id} | Deletes the given addon that belongs to the cluster.
[**delete_addon_v2**](AddonApi.md#delete_addon_v2) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/addons/{addon_id} | Deletes the given addon that belongs to the cluster.
[**get_addon**](AddonApi.md#get_addon) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/addons/{addon_id} | Gets an addon that is assigned to the given cluster.
[**get_addon_v2**](AddonApi.md#get_addon_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/addons/{addon_id} | Gets an addon that is assigned to the given cluster.
[**list_accessible_addons**](AddonApi.md#list_accessible_addons) | **POST** /api/v1/addons | 
[**list_addons**](AddonApi.md#list_addons) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/addons | 
[**list_addons_v2**](AddonApi.md#list_addons_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/addons | 
[**list_installable_addons**](AddonApi.md#list_installable_addons) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/installableaddons | 
[**list_installable_addons_v2**](AddonApi.md#list_installable_addons_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/installableaddons | 
[**patch_addon**](AddonApi.md#patch_addon) | **PATCH** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/addons/{addon_id} | Patches an addon that is assigned to the given cluster.
[**patch_addon_v2**](AddonApi.md#patch_addon_v2) | **PATCH** /api/v2/projects/{project_id}/clusters/{cluster_id}/addons/{addon_id} | Patches an addon that is assigned to the given cluster.



## create_addon

> crate::models::Addon create_addon(project_id, dc, cluster_id, body)


Creates an addon that will belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**Addon**](Addon.md)> |  |  |

### Return type

[**crate::models::Addon**](Addon.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_addon_v2

> crate::models::Addon create_addon_v2(project_id, cluster_id, body)


Creates an addon that will belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | Option<[**Addon**](Addon.md)> |  |  |

### Return type

[**crate::models::Addon**](Addon.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_addon

> delete_addon(project_id, dc, cluster_id, addon_id)
Deletes the given addon that belongs to the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**addon_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_addon_v2

> delete_addon_v2(project_id, cluster_id, addon_id)
Deletes the given addon that belongs to the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**addon_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addon

> crate::models::Addon get_addon(project_id, dc, cluster_id, addon_id)
Gets an addon that is assigned to the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**addon_id** | **String** |  | [required] |

### Return type

[**crate::models::Addon**](Addon.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addon_v2

> crate::models::Addon get_addon_v2(project_id, cluster_id, addon_id)
Gets an addon that is assigned to the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**addon_id** | **String** |  | [required] |

### Return type

[**crate::models::Addon**](Addon.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_accessible_addons

> crate::models::AccessibleAddons list_accessible_addons()


Lists names of addons that can be configured inside the user clusters

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccessibleAddons**](AccessibleAddons.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_addons

> Vec<crate::models::Addon> list_addons(project_id, dc, cluster_id)


Lists addons that belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Addon>**](Addon.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_addons_v2

> Vec<crate::models::Addon> list_addons_v2(project_id, cluster_id)


Lists addons that belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Addon>**](Addon.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_installable_addons

> crate::models::AccessibleAddons list_installable_addons(project_id, dc, cluster_id)


Lists names of addons that can be installed inside the user cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::AccessibleAddons**](AccessibleAddons.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_installable_addons_v2

> crate::models::AccessibleAddons list_installable_addons_v2(project_id, cluster_id)


Lists names of addons that can be installed inside the user cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::AccessibleAddons**](AccessibleAddons.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_addon

> crate::models::Addon patch_addon(project_id, dc, cluster_id, addon_id, body)
Patches an addon that is assigned to the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**addon_id** | **String** |  | [required] |
**body** | Option<[**Addon**](Addon.md)> |  |  |

### Return type

[**crate::models::Addon**](Addon.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_addon_v2

> crate::models::Addon patch_addon_v2(project_id, cluster_id, addon_id, body)
Patches an addon that is assigned to the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**addon_id** | **String** |  | [required] |
**body** | Option<[**Addon**](Addon.md)> |  |  |

### Return type

[**crate::models::Addon**](Addon.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

