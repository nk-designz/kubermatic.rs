# \EksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_eks_capacity_types**](EksApi.md#list_eks_capacity_types) | **GET** /api/v2/eks/capacitytypes | Gets the EKS Capacity types for node group.
[**list_eks_instance_types_no_credentials**](EksApi.md#list_eks_instance_types_no_credentials) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/eks/instancetypes | Gets the EKS Instance types for node group.
[**list_eks_regions**](EksApi.md#list_eks_regions) | **GET** /api/v2/providers/eks/regions | List EKS regions.
[**list_eks_security_groups**](EksApi.md#list_eks_security_groups) | **GET** /api/v2/providers/eks/securitygroups | List EKS securitygroup list.
[**list_eks_subnets**](EksApi.md#list_eks_subnets) | **GET** /api/v2/providers/eks/subnets | Lists EKS subnet list.
[**list_eks_subnets_no_credentials**](EksApi.md#list_eks_subnets_no_credentials) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/eks/subnets | Gets the EKS Subnets for node group.
[**list_eks_versions**](EksApi.md#list_eks_versions) | **GET** /api/v2/providers/eks/versions | 
[**list_eksami_types**](EksApi.md#list_eksami_types) | **GET** /api/v2/eks/amitypes | Gets the EKS AMI types for node group.
[**list_eksvpcs**](EksApi.md#list_eksvpcs) | **GET** /api/v2/providers/eks/vpcs | 
[**list_eksvpcs_no_credentials**](EksApi.md#list_eksvpcs_no_credentials) | **GET** /api/v2/projects/{project_id}/kubernetes/clusters/{cluster_id}/providers/eks/vpcs | Gets the EKS vpc's for node group.
[**validate_eks_credentials**](EksApi.md#validate_eks_credentials) | **GET** /api/v2/providers/eks/validatecredentials | 



## list_eks_capacity_types

> crate::models::EksCapacityTypeList list_eks_capacity_types()
Gets the EKS Capacity types for node group.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EksCapacityTypeList**](EKSCapacityTypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eks_instance_types_no_credentials

> crate::models::EksInstanceTypeList list_eks_instance_types_no_credentials(project_id, cluster_id)
Gets the EKS Instance types for node group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::EksInstanceTypeList**](EKSInstanceTypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eks_regions

> Vec<crate::models::EksRegionList> list_eks_regions()
List EKS regions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::EksRegionList>**](EKSRegionList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eks_security_groups

> crate::models::EksSecurityGroupList list_eks_security_groups(access_key_id, secret_access_key, credential, region, vpc_id)
List EKS securitygroup list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> |  |  |
**secret_access_key** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**vpc_id** | Option<**String**> |  |  |

### Return type

[**crate::models::EksSecurityGroupList**](EKSSecurityGroupList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eks_subnets

> crate::models::EksSubnetList list_eks_subnets(access_key_id, secret_access_key, credential, region, vpc_id)
Lists EKS subnet list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> |  |  |
**secret_access_key** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**vpc_id** | Option<**String**> |  |  |

### Return type

[**crate::models::EksSubnetList**](EKSSubnetList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eks_subnets_no_credentials

> crate::models::EksSubnetList list_eks_subnets_no_credentials(project_id, cluster_id, vpc_id)
Gets the EKS Subnets for node group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**vpc_id** | Option<**String**> |  |  |

### Return type

[**crate::models::EksSubnetList**](EKSSubnetList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eks_versions

> Vec<crate::models::MasterVersion> list_eks_versions()


Lists EKS versions

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


## list_eksami_types

> crate::models::EksamiTypeList list_eksami_types()
Gets the EKS AMI types for node group.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EksamiTypeList**](EKSAMITypeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eksvpcs

> crate::models::EksvpcList list_eksvpcs(access_key_id, secret_access_key, credential, region)


Lists EKS vpc's

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> |  |  |
**secret_access_key** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |

### Return type

[**crate::models::EksvpcList**](EKSVPCList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_eksvpcs_no_credentials

> crate::models::EksvpcList list_eksvpcs_no_credentials(project_id, cluster_id)
Gets the EKS vpc's for node group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::EksvpcList**](EKSVPCList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_eks_credentials

> validate_eks_credentials(access_key_id, secret_access_key, credential, region)


Validates EKS credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> |  |  |
**secret_access_key** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

