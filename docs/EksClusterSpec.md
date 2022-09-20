# EksClusterSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The Unix epoch timestamp in seconds for when the cluster was created. | [optional]
**kubernetes_network_config** | Option<[**crate::models::EksKubernetesNetworkConfigResponse**](EKSKubernetesNetworkConfigResponse.md)> |  | [optional]
**role_arn** | Option<**String**> | The Amazon Resource Name (ARN) of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf. For more information, see Amazon EKS Service IAM Role (https://docs.aws.amazon.com/eks/latest/userguide/service_IAM_role.html) in the Amazon EKS User Guide .  RoleArn is a required field | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | The metadata that you apply to the cluster to assist with categorization and organization. Each tag consists of a key and an optional value. You define both. Cluster tags do not propagate to any other resources associated with the cluster. | [optional]
**version** | Option<**String**> | The desired Kubernetes version for your cluster. If you don't specify a value here, the latest version available in Amazon EKS is used. | [optional]
**vpc_config_request** | Option<[**crate::models::VpcConfigRequest**](VpcConfigRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


