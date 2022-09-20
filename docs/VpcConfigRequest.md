# VpcConfigRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**security_group_ids** | Option<**Vec<String>**> | Specify one or more security groups for the cross-account elastic network interfaces that Amazon EKS creates to use to allow communication between your nodes and the Kubernetes control plane. For more information, see Amazon EKS security group considerations (https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html) in the Amazon EKS User Guide . | [optional]
**subnet_ids** | Option<**Vec<String>**> | Specify subnets for your Amazon EKS nodes. Amazon EKS creates cross-account elastic network interfaces in these subnets to allow communication between your nodes and the Kubernetes control plane. | [optional]
**vpc_id** | Option<**String**> | The VPC associated with your cluster. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


