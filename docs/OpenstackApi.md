# \OpenstackApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_openstack_availability_zones**](OpenstackApi.md#list_openstack_availability_zones) | **GET** /api/v1/providers/openstack/availabilityzones | 
[**list_openstack_availability_zones_no_credentials**](OpenstackApi.md#list_openstack_availability_zones_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/openstack/availabilityzones | 
[**list_openstack_availability_zones_no_credentials_v2**](OpenstackApi.md#list_openstack_availability_zones_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/openstack/availabilityzones | 
[**list_openstack_networks**](OpenstackApi.md#list_openstack_networks) | **GET** /api/v1/providers/openstack/networks | 
[**list_openstack_networks_no_credentials**](OpenstackApi.md#list_openstack_networks_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/openstack/networks | 
[**list_openstack_networks_no_credentials_v2**](OpenstackApi.md#list_openstack_networks_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/openstack/networks | 
[**list_openstack_security_groups**](OpenstackApi.md#list_openstack_security_groups) | **GET** /api/v1/providers/openstack/securitygroups | 
[**list_openstack_security_groups_no_credentials**](OpenstackApi.md#list_openstack_security_groups_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/openstack/securitygroups | 
[**list_openstack_security_groups_no_credentials_v2**](OpenstackApi.md#list_openstack_security_groups_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/openstack/securitygroups | 
[**list_openstack_sizes**](OpenstackApi.md#list_openstack_sizes) | **GET** /api/v1/providers/openstack/sizes | 
[**list_openstack_sizes_no_credentials**](OpenstackApi.md#list_openstack_sizes_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/openstack/sizes | 
[**list_openstack_sizes_no_credentials_v2**](OpenstackApi.md#list_openstack_sizes_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/openstack/sizes | 
[**list_openstack_subnet_pools**](OpenstackApi.md#list_openstack_subnet_pools) | **GET** /api/v2/providers/openstack/subnetpools | 
[**list_openstack_subnets**](OpenstackApi.md#list_openstack_subnets) | **GET** /api/v1/providers/openstack/subnets | 
[**list_openstack_subnets_no_credentials**](OpenstackApi.md#list_openstack_subnets_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/openstack/subnets | 
[**list_openstack_subnets_no_credentials_v2**](OpenstackApi.md#list_openstack_subnets_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/openstack/subnets | 
[**list_openstack_tenants**](OpenstackApi.md#list_openstack_tenants) | **GET** /api/v1/providers/openstack/tenants | 
[**list_openstack_tenants_no_credentials**](OpenstackApi.md#list_openstack_tenants_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/openstack/tenants | 
[**list_openstack_tenants_no_credentials_v2**](OpenstackApi.md#list_openstack_tenants_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/openstack/tenants | 



## list_openstack_availability_zones

> Vec<crate::models::OpenstackAvailabilityZone> list_openstack_availability_zones(username, password, domain, tenant, tenant_id, project, project_id, datacenter_name, application_credential_id, application_credential_secret, oidc_authentication, credential)


Lists availability zones from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**application_credential_id** | Option<**String**> |  |  |
**application_credential_secret** | Option<**String**> |  |  |
**oidc_authentication** | Option<**bool**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OpenstackAvailabilityZone>**](OpenstackAvailabilityZone.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_availability_zones_no_credentials

> Vec<crate::models::OpenstackAvailabilityZone> list_openstack_availability_zones_no_credentials(project_id, dc, cluster_id)


Lists availability zones from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackAvailabilityZone>**](OpenstackAvailabilityZone.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_availability_zones_no_credentials_v2

> Vec<crate::models::OpenstackAvailabilityZone> list_openstack_availability_zones_no_credentials_v2(project_id, cluster_id)


Lists availability zones from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackAvailabilityZone>**](OpenstackAvailabilityZone.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_networks

> Vec<crate::models::OpenstackNetwork> list_openstack_networks(username, password, domain, tenant, tenant_id, project, project_id, datacenter_name, application_credential_id, application_credential_secret, oidc_authentication, credential)


Lists networks from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**application_credential_id** | Option<**String**> |  |  |
**application_credential_secret** | Option<**String**> |  |  |
**oidc_authentication** | Option<**bool**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OpenstackNetwork>**](OpenstackNetwork.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_networks_no_credentials

> Vec<crate::models::OpenstackNetwork> list_openstack_networks_no_credentials(project_id, dc, cluster_id)


Lists networks from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackNetwork>**](OpenstackNetwork.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_networks_no_credentials_v2

> Vec<crate::models::OpenstackNetwork> list_openstack_networks_no_credentials_v2(project_id, cluster_id)


Lists networks from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackNetwork>**](OpenstackNetwork.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_security_groups

> Vec<crate::models::OpenstackSecurityGroup> list_openstack_security_groups(username, password, domain, tenant, tenant_id, project, project_id, datacenter_name, application_credential_id, application_credential_secret, oidc_authentication, credential)


Lists security groups from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**application_credential_id** | Option<**String**> |  |  |
**application_credential_secret** | Option<**String**> |  |  |
**oidc_authentication** | Option<**bool**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OpenstackSecurityGroup>**](OpenstackSecurityGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_security_groups_no_credentials

> Vec<crate::models::OpenstackSecurityGroup> list_openstack_security_groups_no_credentials(project_id, dc, cluster_id)


Lists security groups from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackSecurityGroup>**](OpenstackSecurityGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_security_groups_no_credentials_v2

> Vec<crate::models::OpenstackSecurityGroup> list_openstack_security_groups_no_credentials_v2(project_id, cluster_id)


Lists security groups from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackSecurityGroup>**](OpenstackSecurityGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_sizes

> Vec<crate::models::OpenstackSize> list_openstack_sizes(username, password, domain, tenant, tenant_id, project, project_id, datacenter_name, application_credential_id, application_credential_secret, oidc_authentication, credential)


Lists sizes from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**application_credential_id** | Option<**String**> |  |  |
**application_credential_secret** | Option<**String**> |  |  |
**oidc_authentication** | Option<**bool**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OpenstackSize>**](OpenstackSize.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_sizes_no_credentials

> Vec<crate::models::OpenstackSize> list_openstack_sizes_no_credentials(project_id, dc, cluster_id)


Lists sizes from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackSize>**](OpenstackSize.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_sizes_no_credentials_v2

> Vec<crate::models::OpenstackSize> list_openstack_sizes_no_credentials_v2(project_id, cluster_id)


Lists sizes from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackSize>**](OpenstackSize.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_subnet_pools

> Vec<crate::models::OpenstackSubnetPool> list_openstack_subnet_pools(username, password, domain, tenant, tenant_id, project, project_id, datacenter_name, application_credential_id, application_credential_secret, oidc_authentication, credential, ip_version)


Lists subnet pools from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**application_credential_id** | Option<**String**> |  |  |
**application_credential_secret** | Option<**String**> |  |  |
**oidc_authentication** | Option<**bool**> |  |  |
**credential** | Option<**String**> |  |  |
**ip_version** | Option<**i64**> |  |  |

### Return type

[**Vec<crate::models::OpenstackSubnetPool>**](OpenstackSubnetPool.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_subnets

> Vec<crate::models::OpenstackSubnet> list_openstack_subnets(username, password, domain, tenant, tenant_id, project, project_id, datacenter_name, application_credential_id, application_credential_secret, oidc_authentication, credential, network_id)


Lists subnets from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**application_credential_id** | Option<**String**> |  |  |
**application_credential_secret** | Option<**String**> |  |  |
**oidc_authentication** | Option<**bool**> |  |  |
**credential** | Option<**String**> |  |  |
**network_id** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OpenstackSubnet>**](OpenstackSubnet.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_subnets_no_credentials

> Vec<crate::models::OpenstackSubnet> list_openstack_subnets_no_credentials(project_id, dc, cluster_id, network_id)


Lists subnets from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**network_id** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OpenstackSubnet>**](OpenstackSubnet.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_subnets_no_credentials_v2

> Vec<crate::models::OpenstackSubnet> list_openstack_subnets_no_credentials_v2(project_id, cluster_id, network_id)


Lists subnets from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**network_id** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OpenstackSubnet>**](OpenstackSubnet.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_tenants

> Vec<crate::models::OpenstackTenant> list_openstack_tenants(username, password, domain, datacenter_name, application_credential_id, application_credential_secret, oidc_authentication, credential)


Lists tenants from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**datacenter_name** | Option<**String**> |  |  |
**application_credential_id** | Option<**String**> |  |  |
**application_credential_secret** | Option<**String**> |  |  |
**oidc_authentication** | Option<**bool**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::OpenstackTenant>**](OpenstackTenant.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_tenants_no_credentials

> Vec<crate::models::OpenstackTenant> list_openstack_tenants_no_credentials(project_id, dc, cluster_id)


Lists tenants from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackTenant>**](OpenstackTenant.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_openstack_tenants_no_credentials_v2

> Vec<crate::models::OpenstackTenant> list_openstack_tenants_no_credentials_v2(project_id, cluster_id)


Lists tenants from openstack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OpenstackTenant>**](OpenstackTenant.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

