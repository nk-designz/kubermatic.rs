# GkeNodePoolAutoscaling

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoprovisioned** | Option<**bool**> | Autoprovisioned: Can this node pool be deleted automatically. | [optional]
**enabled** | Option<**bool**> | Enabled: Is autoscaling enabled for this node pool. | [optional]
**max_node_count** | Option<**i64**> | MaxNodeCount: Maximum number of nodes in the NodePool. Must be >= min_node_count. There has to enough quota to scale up the cluster. | [optional]
**min_node_count** | Option<**i64**> | MinNodeCount: Minimum number of nodes in the NodePool. Must be >= 1 and <= max_node_count. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


