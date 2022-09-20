# \MetricApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_machine_deployment_metrics**](MetricApi.md#list_machine_deployment_metrics) | **GET** /api/v2/projects/{project_id}/clusters/{cluster_id}/machinedeployments/{machinedeployment_id}/nodes/metrics | Lists metrics that belong to the given machine deployment.
[**list_node_deployment_metrics**](MetricApi.md#list_node_deployment_metrics) | **GET** /api/v1/projects/{project_id}/dc/{dc}/clusters/{cluster_id}/nodedeployments/{nodedeployment_id}/nodes/metrics | Lists metrics that belong to the given node deployment.



## list_machine_deployment_metrics

> Vec<crate::models::NodeMetric> list_machine_deployment_metrics(project_id, cluster_id, machinedeployment_id)
Lists metrics that belong to the given machine deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**machinedeployment_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NodeMetric>**](NodeMetric.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_node_deployment_metrics

> Vec<crate::models::NodeMetric> list_node_deployment_metrics(project_id, dc, cluster_id, nodedeployment_id)
Lists metrics that belong to the given node deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**dc** | **String** |  | [required] |
**cluster_id** | **String** |  | [required] |
**nodedeployment_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NodeMetric>**](NodeMetric.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

