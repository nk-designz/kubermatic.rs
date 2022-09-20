# \AzureApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_azure_availability_zones**](AzureApi.md#list_azure_availability_zones) | **GET** /api/v1/providers/azure/availabilityzones | 
[**list_azure_availability_zones_no_credentials**](AzureApi.md#list_azure_availability_zones_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/azure/availabilityzones | 
[**list_azure_availability_zones_no_credentials_v2**](AzureApi.md#list_azure_availability_zones_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/azure/availabilityzones | 
[**list_azure_resource_groups**](AzureApi.md#list_azure_resource_groups) | **GET** /api/v2/providers/azure/resourcegroups | 
[**list_azure_route_tables**](AzureApi.md#list_azure_route_tables) | **GET** /api/v2/providers/azure/routetables | 
[**list_azure_security_groups**](AzureApi.md#list_azure_security_groups) | **GET** /api/v2/providers/azure/securitygroups | 
[**list_azure_sizes**](AzureApi.md#list_azure_sizes) | **GET** /api/v1/providers/azure/sizes | 
[**list_azure_sizes_no_credentials**](AzureApi.md#list_azure_sizes_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/azure/sizes | 
[**list_azure_sizes_no_credentials_v2**](AzureApi.md#list_azure_sizes_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/azure/sizes | 
[**list_azure_subnets**](AzureApi.md#list_azure_subnets) | **GET** /api/v2/providers/azure/subnets | 
[**list_azure_vnets**](AzureApi.md#list_azure_vnets) | **GET** /api/v2/providers/azure/vnets | 



## list_azure_availability_zones

> crate::models::AzureAvailabilityZonesList list_azure_availability_zones()


Lists VM availability zones in an Azure region

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AzureAvailabilityZonesList**](AzureAvailabilityZonesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_availability_zones_no_credentials

> crate::models::AzureAvailabilityZonesList list_azure_availability_zones_no_credentials(project_id, dc, cluster_id, sku_name)


Lists available VM availability zones in an Azure region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**sku_name** | Option<**String**> |  |  |

### Return type

[**crate::models::AzureAvailabilityZonesList**](AzureAvailabilityZonesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_availability_zones_no_credentials_v2

> crate::models::AzureAvailabilityZonesList list_azure_availability_zones_no_credentials_v2(project_id, cluster_id, sku_name)


Lists available VM availability zones in an Azure region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**sku_name** | Option<**String**> |  |  |

### Return type

[**crate::models::AzureAvailabilityZonesList**](AzureAvailabilityZonesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_resource_groups

> crate::models::AzureResourceGroupsList list_azure_resource_groups(subscription_id, tenant_id, client_id, client_secret, credential, location)


Lists available VM resource groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |

### Return type

[**crate::models::AzureResourceGroupsList**](AzureResourceGroupsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_route_tables

> crate::models::AzureRouteTablesList list_azure_route_tables(subscription_id, tenant_id, client_id, client_secret, credential, resource_group, location)


Lists available VM route tables

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**resource_group** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |

### Return type

[**crate::models::AzureRouteTablesList**](AzureRouteTablesList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_security_groups

> crate::models::AzureSecurityGroupsList list_azure_security_groups(subscription_id, tenant_id, client_id, client_secret, credential, resource_group, location)


Lists available VM security groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**resource_group** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |

### Return type

[**crate::models::AzureSecurityGroupsList**](AzureSecurityGroupsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_sizes

> Vec<crate::models::AzureSize> list_azure_sizes(subscription_id, tenant_id, client_id, client_secret, location, credential)


Lists available VM sizes in an Azure region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::AzureSize>**](AzureSize.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_sizes_no_credentials

> Vec<crate::models::AzureSize> list_azure_sizes_no_credentials(project_id, dc, cluster_id)


Lists available VM sizes in an Azure region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::AzureSize>**](AzureSize.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_sizes_no_credentials_v2

> Vec<crate::models::AzureSize> list_azure_sizes_no_credentials_v2(project_id, cluster_id)


Lists available VM sizes in an Azure region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::AzureSize>**](AzureSize.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_subnets

> crate::models::AzureSubnetsList list_azure_subnets(subscription_id, tenant_id, client_id, client_secret, credential, resource_group, virtual_network)


Lists available VM subnets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**resource_group** | Option<**String**> |  |  |
**virtual_network** | Option<**String**> |  |  |

### Return type

[**crate::models::AzureSubnetsList**](AzureSubnetsList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_vnets

> crate::models::AzureVirtualNetworksList list_azure_vnets(subscription_id, tenant_id, client_id, client_secret, credential, resource_group, location)


Lists available VM virtual networks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**resource_group** | Option<**String**> |  |  |
**location** | Option<**String**> |  |  |

### Return type

[**crate::models::AzureVirtualNetworksList**](AzureVirtualNetworksList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

