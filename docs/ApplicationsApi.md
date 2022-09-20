# \ApplicationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_application_installation**](ApplicationsApi.md#create_application_installation) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/applicationinstallations | 
[**delete_application_installation**](ApplicationsApi.md#delete_application_installation) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/applicationinstallations/{namespace}/{appinstall_name} | 
[**get_application_installation**](ApplicationsApi.md#get_application_installation) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/applicationinstallations/{namespace}/{appinstall_name} | 
[**list_application_definitions**](ApplicationsApi.md#list_application_definitions) | **GET** /api/v2/applicationdefinitions | 
[**list_application_installations**](ApplicationsApi.md#list_application_installations) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/applicationinstallations | 
[**update_application_installation**](ApplicationsApi.md#update_application_installation) | **PUT** /api/v2/projects/{project_id}/clusters/{cluster_id}/applicationinstallations/{namespace}/{appinstall_name} | 



## create_application_installation

> crate::models::ApplicationInstallation create_application_installation(project_id, cluster_id, body)


Creates ApplicationInstallation into the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | [**ApplicationInstallationBody**](ApplicationInstallationBody.md) |  | [required] |

### Return type

[**crate::models::ApplicationInstallation**](ApplicationInstallation.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_installation

> delete_application_installation(project_id, cluster_id, namespace, appinstall_name)


Deletes the given ApplicationInstallation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**appinstall_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_installation

> crate::models::ApplicationInstallation get_application_installation(project_id, cluster_id, namespace, appinstall_name)


Gets the given ApplicationInstallation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**appinstall_name** | **String** |  | [required] |

### Return type

[**crate::models::ApplicationInstallation**](ApplicationInstallation.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_definitions

> Vec<crate::models::ApplicationDefinition> list_application_definitions()


List ApplicationDefinitions which are available in the KKP installation

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ApplicationDefinition>**](ApplicationDefinition.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_installations

> Vec<crate::models::ApplicationInstallation> list_application_installations(project_id, cluster_id)


List ApplicationInstallations which belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ApplicationInstallation>**](ApplicationInstallation.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_installation

> crate::models::ApplicationInstallation update_application_installation(project_id, cluster_id, namespace, appinstall_name, body)


Updates the given ApplicationInstallation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**appinstall_name** | **String** |  | [required] |
**body** | [**ApplicationInstallationBody**](ApplicationInstallationBody.md) |  | [required] |

### Return type

[**crate::models::ApplicationInstallation**](ApplicationInstallation.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

