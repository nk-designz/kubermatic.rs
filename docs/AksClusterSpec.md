# AksClusterSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The timestamp of resource creation (UTC). | [optional]
**created_by** | Option<**String**> | The identity that created the resource. | [optional]
**dns_prefix** | Option<**String**> | DNSPrefix - This cannot be updated once the Managed Cluster has been created. | [optional]
**enable_rbac** | Option<**bool**> | EnableRBAC - Whether Kubernetes Role-Based Access Control Enabled. | [optional]
**fqdn** | Option<**String**> | Fqdn - READ-ONLY; The FQDN of the master pool. | [optional]
**fqdn_subdomain** | Option<**String**> | FqdnSubdomain - This cannot be updated once the Managed Cluster has been created. | [optional]
**kubernetes_version** | Option<**String**> | KubernetesVersion - When you upgrade a supported AKS cluster, Kubernetes minor versions cannot be skipped. All upgrades must be performed sequentially by major version number. For example, upgrades between 1.14.x -> 1.15.x or 1.15.x -> 1.16.x are allowed, however 1.14.x -> 1.16.x is not allowed. See [upgrading an AKS cluster](https://docs.microsoft.com/azure/aks/upgrade-cluster) for more details. | [optional]
**location** | Option<**String**> | Location - Resource location | [optional]
**machine_deployment_spec** | Option<[**crate::models::AksMachineDeploymentCloudSpec**](AKSMachineDeploymentCloudSpec.md)> |  | [optional]
**network_profile** | Option<[**crate::models::AksNetworkProfile**](AKSNetworkProfile.md)> |  | [optional]
**private_fqdn** | Option<**String**> | PrivateFQDN - READ-ONLY; The FQDN of private cluster. | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Resource tags. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


