# \VersionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_kubermatic_version**](VersionsApi.md#get_kubermatic_version) | **GET** /api/v1/version | Get versions of running Kubermatic components.
[**get_master_versions**](VersionsApi.md#get_master_versions) | **GET** /api/v1/upgrades/cluster | 
[**get_node_upgrades**](VersionsApi.md#get_node_upgrades) | **GET** /api/v1/upgrades/node | 



## get_kubermatic_version

> crate::models::KubermaticVersions get_kubermatic_version()
Get versions of running Kubermatic components.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::KubermaticVersions**](KubermaticVersions.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_master_versions

> Vec<crate::models::MasterVersion> get_master_versions()


Lists all versions which don't result in automatic updates

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


## get_node_upgrades

> Vec<crate::models::MasterVersion> get_node_upgrades(_type, control_plane_version)


Gets possible node upgrades for a specific control plane version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<**String**> | Type is deprecated and not used anymore. |  |
**control_plane_version** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::MasterVersion>**](MasterVersion.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

