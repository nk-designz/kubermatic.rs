# GkeClusterAutoscaling

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoprovisioning_locations** | Option<**Vec<String>**> | AutoprovisioningLocations: The list of Google Compute Engine zones (https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes can be created by NAP. | [optional]
**autoprovisioning_node_pool_defaults** | Option<[**crate::models::GkeAutoprovisioningNodePoolDefaults**](GKEAutoprovisioningNodePoolDefaults.md)> |  | [optional]
**enable_node_autoprovisioning** | Option<**bool**> | EnableNodeAutoprovisioning: Enables automatic node pool creation and deletion. | [optional]
**resource_limits** | Option<[**Vec<crate::models::GkeResourceLimit>**](GKEResourceLimit.md)> | ResourceLimits: Contains global constraints regarding minimum and maximum amount of resources in the cluster. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


