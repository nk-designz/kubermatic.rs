# EksNodegroupScalingConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**desired_size** | Option<**i64**> | The current number of nodes that the managed node group should maintain. | [optional]
**max_size** | Option<**i64**> | The maximum number of nodes that the managed node group can scale out to. For information about the maximum number that you can specify, see Amazon EKS service quotas (https://docs.aws.amazon.com/eks/latest/userguide/service-quotas.html) in the Amazon EKS User Guide. | [optional]
**min_size** | Option<**i64**> | The minimum number of nodes that the managed node group can scale in to. This number must be greater than zero. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


