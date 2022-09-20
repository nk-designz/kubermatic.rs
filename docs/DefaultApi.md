# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_oidc_kubeconfig**](DefaultApi.md#create_oidc_kubeconfig) | **GET** /api/v1/kubeconfig | 
[**create_oidc_kubeconfig_secret**](DefaultApi.md#create_oidc_kubeconfig_secret) | **GET** /api/v2/kubeconfig/secret | 
[**get_addon_config**](DefaultApi.md#get_addon_config) | **GET** /api/v1/addonconfigs/{addon_id} | Returns specified addon config.
[**get_admission_plugins**](DefaultApi.md#get_admission_plugins) | **GET** /api/v1/admission/plugins/{version} | Returns specified addon config.
[**list_addon_configs**](DefaultApi.md#list_addon_configs) | **GET** /api/v1/addonconfigs | Returns all available addon configs.
[**list_system_labels**](DefaultApi.md#list_system_labels) | **GET** /api/v1/labels/system | 
[**migrate_cluster_to_external_ccm**](DefaultApi.md#migrate_cluster_to_external_ccm) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/externalccmmigration | 



## create_oidc_kubeconfig

> Vec<i32> create_oidc_kubeconfig(cluster_id, project_id, user_id)


Starts OIDC flow and generates kubeconfig, the generated config contains OIDC provider authentication info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**user_id** | Option<**String**> |  |  |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_oidc_kubeconfig_secret

> create_oidc_kubeconfig_secret()


Starts OIDC flow and generates kubeconfig, the generated config contains OIDC provider authentication info. The kubeconfig is stored in the secret.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addon_config

> crate::models::AddonConfig get_addon_config(addon_id)
Returns specified addon config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_id** | **String** |  | [required] |

### Return type

[**crate::models::AddonConfig**](AddonConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_admission_plugins

> Vec<String> get_admission_plugins(version)
Returns specified addon config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_addon_configs

> Vec<crate::models::AddonConfig> list_addon_configs()
Returns all available addon configs.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AddonConfig>**](AddonConfig.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_system_labels

> ::std::collections::HashMap<String, crate::models::Array> list_system_labels()


List restricted system labels

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::Array>**](array.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_cluster_to_external_ccm

> migrate_cluster_to_external_ccm(project_id, cluster_id)


Enable the migration to the external CCM for the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

