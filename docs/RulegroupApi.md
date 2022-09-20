# \RulegroupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_admin_rule_group**](RulegroupApi.md#create_admin_rule_group) | **POST** /api/v2/seeds/{seed_name}/rulegroups | 
[**create_rule_group**](RulegroupApi.md#create_rule_group) | **POST** /api/v2/projects/{project_id}/clusters/{cluster_id}/rulegroups | 
[**delete_admin_rule_group**](RulegroupApi.md#delete_admin_rule_group) | **DELETE** /api/v2/seeds/{seed_name}/rulegroups/{rulegroup_id} | Deletes the given rule group that belongs to the Seed.
[**delete_rule_group**](RulegroupApi.md#delete_rule_group) | **DELETE** /api/v2/projects/{project_id}/clusters/{cluster_id}/rulegroups/{rulegroup_id} | Deletes the given rule group that belongs to the cluster.
[**get_admin_rule_group**](RulegroupApi.md#get_admin_rule_group) | **GET** /api/v2/seeds/{seed_name}/rulegroups/{rulegroup_id} | Gets a specified rule group for a given Seed.
[**get_rule_group**](RulegroupApi.md#get_rule_group) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/rulegroups/{rulegroup_id} | Gets a specified rule group for the given cluster.
[**list_admin_rule_groups**](RulegroupApi.md#list_admin_rule_groups) | **GET** /api/v2/seeds/{seed_name}/rulegroups | Lists rule groups that belong to a given Seed.
[**list_rule_groups**](RulegroupApi.md#list_rule_groups) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/rulegroups | 
[**update_admin_rule_group**](RulegroupApi.md#update_admin_rule_group) | **PUT** /api/v2/seeds/{seed_name}/rulegroups/{rulegroup_id} | Updates the specified rule group for the given Seed.
[**update_rule_group**](RulegroupApi.md#update_rule_group) | **PUT** /api/v2/projects/{project_id}/clusters/{cluster_id}/rulegroups/{rulegroup_id} | Updates the specified rule group for the given cluster.



## create_admin_rule_group

> crate::models::RuleGroup create_admin_rule_group(seed_name, body)


Creates a rule group that will belong to the given Seed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**body** | [**RuleGroup**](RuleGroup.md) |  | [required] |

### Return type

[**crate::models::RuleGroup**](RuleGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_rule_group

> crate::models::RuleGroup create_rule_group(project_id, cluster_id, body)


Creates a rule group that will belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**body** | [**RuleGroup**](RuleGroup.md) |  | [required] |

### Return type

[**crate::models::RuleGroup**](RuleGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_admin_rule_group

> delete_admin_rule_group(seed_name, rulegroup_id)
Deletes the given rule group that belongs to the Seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**rulegroup_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rule_group

> delete_rule_group(project_id, cluster_id, rulegroup_id)
Deletes the given rule group that belongs to the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**rulegroup_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_admin_rule_group

> crate::models::RuleGroup get_admin_rule_group(seed_name, rulegroup_id)
Gets a specified rule group for a given Seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**rulegroup_id** | **String** |  | [required] |

### Return type

[**crate::models::RuleGroup**](RuleGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_group

> crate::models::RuleGroup get_rule_group(project_id, cluster_id, rulegroup_id)
Gets a specified rule group for the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**rulegroup_id** | **String** |  | [required] |

### Return type

[**crate::models::RuleGroup**](RuleGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_admin_rule_groups

> Vec<crate::models::RuleGroup> list_admin_rule_groups(seed_name, _type)
Lists rule groups that belong to a given Seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::RuleGroup>**](RuleGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rule_groups

> Vec<crate::models::RuleGroup> list_rule_groups(project_id, cluster_id, _type)


Lists rule groups that belong to the given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::RuleGroup>**](RuleGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_admin_rule_group

> crate::models::RuleGroup update_admin_rule_group(seed_name, rulegroup_id, body)
Updates the specified rule group for the given Seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**rulegroup_id** | **String** |  | [required] |
**body** | [**RuleGroup**](RuleGroup.md) |  | [required] |

### Return type

[**crate::models::RuleGroup**](RuleGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rule_group

> crate::models::RuleGroup update_rule_group(project_id, cluster_id, rulegroup_id, body)
Updates the specified rule group for the given cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**rulegroup_id** | **String** |  | [required] |
**body** | [**RuleGroup**](RuleGroup.md) |  | [required] |

### Return type

[**crate::models::RuleGroup**](RuleGroup.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

