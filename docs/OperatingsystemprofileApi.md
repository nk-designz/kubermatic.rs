# \OperatingsystemprofileApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_operating_system_profiles**](OperatingsystemprofileApi.md#list_operating_system_profiles) | **GET** /api/v2/seeds/{seed_name}/operatingsystemprofiles | Lists Operating System Profiles.
[**list_operating_system_profiles_for_cluster**](OperatingsystemprofileApi.md#list_operating_system_profiles_for_cluster) | **GET** /projects/{project_id}/clusters/{cluster_id}/operatingsystemprofiles | 



## list_operating_system_profiles

> Vec<crate::models::OperatingSystemProfile> list_operating_system_profiles(seed_name)
Lists Operating System Profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OperatingSystemProfile>**](OperatingSystemProfile.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_operating_system_profiles_for_cluster

> Vec<crate::models::OperatingSystemProfile> list_operating_system_profiles_for_cluster(project_id, cluster_id)


Lists all available Operating System Profiles for a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::OperatingSystemProfile>**](OperatingSystemProfile.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

