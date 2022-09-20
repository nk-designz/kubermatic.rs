# GkeMachineDeploymentCloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoscaling** | Option<[**crate::models::GkeNodePoolAutoscaling**](GKENodePoolAutoscaling.md)> |  | [optional]
**config** | Option<[**crate::models::GkeNodeConfig**](GKENodeConfig.md)> |  | [optional]
**locations** | Option<**Vec<String>**> | Locations: The list of Google Compute Engine zones (https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes should be located. If this value is unspecified during node pool creation, the Cluster.Locations (https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations) value will be used, instead. Warning: changing node pool locations will result in nodes being added and/or removed. | [optional]
**management** | Option<[**crate::models::GkeNodeManagement**](GKENodeManagement.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


