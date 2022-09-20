# \AwsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_aws_security_groups**](AwsApi.md#list_aws_security_groups) | **GET** /api/v1/providers/aws/{dc}/securitygroups | 
[**list_aws_sizes**](AwsApi.md#list_aws_sizes) | **GET** /api/v1/providers/aws/sizes | Lists available AWS sizes.
[**list_aws_sizes_no_credentials**](AwsApi.md#list_aws_sizes_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/aws/sizes | 
[**list_aws_sizes_no_credentials_v2**](AwsApi.md#list_aws_sizes_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/aws/sizes | 
[**list_aws_subnets**](AwsApi.md#list_aws_subnets) | **GET** /api/v1/providers/aws/{dc}/subnets | 
[**list_aws_subnets_no_credentials**](AwsApi.md#list_aws_subnets_no_credentials) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/providers/aws/subnets | 
[**list_aws_subnets_no_credentials_v2**](AwsApi.md#list_aws_subnets_no_credentials_v2) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/aws/subnets | 
[**list_awsvpcs**](AwsApi.md#list_awsvpcs) | **GET** /api/v1/providers/aws/{dc}/vpcs | 



## list_aws_security_groups

> crate::models::AwsSecurityGroupList list_aws_security_groups(dc, access_key_id, secret_access_key, credential, assume_role_arn, assume_role_external_id, VPC)


Lists available AWS Security Groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** |  | [required] |
**access_key_id** | Option<**String**> |  |  |
**secret_access_key** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**assume_role_arn** | Option<**String**> |  |  |
**assume_role_external_id** | Option<**String**> |  |  |
**VPC** | Option<**String**> |  |  |

### Return type

[**crate::models::AwsSecurityGroupList**](AWSSecurityGroupList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_sizes

> crate::models::AwsSizeList list_aws_sizes(region, architecture)
Lists available AWS sizes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | Option<**String**> |  |  |
**architecture** | Option<**String**> | architecture query parameter. Supports: arm64 and x64 types. |  |

### Return type

[**crate::models::AwsSizeList**](AWSSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_sizes_no_credentials

> crate::models::AwsSizeList list_aws_sizes_no_credentials(project_id, dc, cluster_id)


Lists available AWS sizes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::AwsSizeList**](AWSSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_sizes_no_credentials_v2

> crate::models::AwsSizeList list_aws_sizes_no_credentials_v2(project_id, cluster_id, architecture)


Lists available AWS sizes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**architecture** | Option<**String**> | architecture query parameter. Supports: arm64 and x64 types. |  |

### Return type

[**crate::models::AwsSizeList**](AWSSizeList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_subnets

> crate::models::AwsSubnetList list_aws_subnets(dc, access_key_id, secret_access_key, credential, assume_role_arn, assume_role_external_id, VPC)


Lists available AWS subnets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** |  | [required] |
**access_key_id** | Option<**String**> |  |  |
**secret_access_key** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**assume_role_arn** | Option<**String**> |  |  |
**assume_role_external_id** | Option<**String**> |  |  |
**VPC** | Option<**String**> |  |  |

### Return type

[**crate::models::AwsSubnetList**](AWSSubnetList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_subnets_no_credentials

> crate::models::AwsSubnetList list_aws_subnets_no_credentials(project_id, dc, cluster_id)


Lists available AWS subnets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::AwsSubnetList**](AWSSubnetList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_subnets_no_credentials_v2

> crate::models::AwsSubnetList list_aws_subnets_no_credentials_v2(project_id, cluster_id)


Lists available AWS subnets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**crate::models::AwsSubnetList**](AWSSubnetList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_awsvpcs

> crate::models::AwsvpcList list_awsvpcs(dc, access_key_id, secret_access_key, credential, assume_role_arn, assume_role_external_id, VPC)


Lists available AWS vpc's

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** |  | [required] |
**access_key_id** | Option<**String**> |  |  |
**secret_access_key** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**assume_role_arn** | Option<**String**> |  |  |
**assume_role_external_id** | Option<**String**> |  |  |
**VPC** | Option<**String**> |  |  |

### Return type

[**crate::models::AwsvpcList**](AWSVPCList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

