# \NutanixApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_nutanix_categories**](NutanixApi.md#list_nutanix_categories) | **GET** /api/v2/providers/nutanix/{dc}/categories | 
[**list_nutanix_categories_no_credentials**](NutanixApi.md#list_nutanix_categories_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/nutanix/categories | 
[**list_nutanix_category_values**](NutanixApi.md#list_nutanix_category_values) | **GET** /api/v2/providers/nutanix/{dc}/categories/{category}/values | 
[**list_nutanix_category_values_no_credentials**](NutanixApi.md#list_nutanix_category_values_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/nutanix/categories/{category}/values | 
[**list_nutanix_clusters**](NutanixApi.md#list_nutanix_clusters) | **GET** /api/v2/providers/nutanix/{dc}/clusters | 
[**list_nutanix_projects**](NutanixApi.md#list_nutanix_projects) | **GET** /api/v2/providers/nutanix/{dc}/projects | 
[**list_nutanix_subnets**](NutanixApi.md#list_nutanix_subnets) | **GET** /api/v2/providers/nutanix/{dc}/subnets | 
[**list_nutanix_subnets_no_credentials**](NutanixApi.md#list_nutanix_subnets_no_credentials) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/providers/nutanix/subnets | 



## list_nutanix_categories

> Vec<crate::models::NutanixCategory> list_nutanix_categories(dc, nutanix_username, nutanix_password, nutanix_proxy_url, credential)


List category keys from Nutanix

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**nutanix_username** | Option<**String**> |  |  |
**nutanix_password** | Option<**String**> |  |  |
**nutanix_proxy_url** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::NutanixCategory>**](NutanixCategory.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nutanix_categories_no_credentials

> Vec<crate::models::NutanixCategory> list_nutanix_categories_no_credentials(project_id, cluster_id)


Lists available Nutanix categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NutanixCategory>**](NutanixCategory.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nutanix_category_values

> Vec<crate::models::NutanixCategoryValue> list_nutanix_category_values(dc, category, nutanix_username, nutanix_password, nutanix_proxy_url, credential)


List available category values for a specific category from Nutanix

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**category** | **String** | Category to query the available values for | [required] |
**nutanix_username** | Option<**String**> |  |  |
**nutanix_password** | Option<**String**> |  |  |
**nutanix_proxy_url** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::NutanixCategoryValue>**](NutanixCategoryValue.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nutanix_category_values_no_credentials

> Vec<crate::models::NutanixCategoryValue> list_nutanix_category_values_no_credentials(project_id, cluster_id, category)


Lists available Nutanix category values for a specific category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**category** | **String** | Category to query the available values for | [required] |

### Return type

[**Vec<crate::models::NutanixCategoryValue>**](NutanixCategoryValue.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nutanix_clusters

> Vec<crate::models::NutanixCluster> list_nutanix_clusters(dc, nutanix_username, nutanix_password, nutanix_proxy_url, credential)


List clusters from Nutanix

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**nutanix_username** | Option<**String**> |  |  |
**nutanix_password** | Option<**String**> |  |  |
**nutanix_proxy_url** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::NutanixCluster>**](NutanixCluster.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nutanix_projects

> Vec<crate::models::NutanixProject> list_nutanix_projects(dc, nutanix_username, nutanix_password, nutanix_proxy_url, credential)


List projects from Nutanix

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**nutanix_username** | Option<**String**> |  |  |
**nutanix_password** | Option<**String**> |  |  |
**nutanix_proxy_url** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::NutanixProject>**](NutanixProject.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nutanix_subnets

> Vec<crate::models::NutanixSubnet> list_nutanix_subnets(dc, nutanix_cluster, nutanix_username, nutanix_password, nutanix_proxy_url, credential, nutanix_project)


List subnets from Nutanix

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dc** | **String** | KKP Datacenter to use for endpoint | [required] |
**nutanix_cluster** | **String** |  | [required] |
**nutanix_username** | Option<**String**> |  |  |
**nutanix_password** | Option<**String**> |  |  |
**nutanix_proxy_url** | Option<**String**> |  |  |
**credential** | Option<**String**> |  |  |
**nutanix_project** | Option<**String**> | Project query parameter. Can be omitted to query subnets without project scope |  |

### Return type

[**Vec<crate::models::NutanixSubnet>**](NutanixSubnet.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nutanix_subnets_no_credentials

> Vec<crate::models::NutanixSubnet> list_nutanix_subnets_no_credentials(project_id, cluster_id)


Lists available Nutanix Subnets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NutanixSubnet>**](NutanixSubnet.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

