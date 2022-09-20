# AgentPoolBasics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability_zones** | Option<**Vec<String>**> | AvailabilityZones - The list of Availability zones to use for nodes. This can only be specified if the AgentPoolType property is 'VirtualMachineScaleSets'. | [optional]
**count** | Option<**i32**> | Required: Count - Number of agents (VMs) to host docker containers. Allowed values must be in the range of 0 to 1000 (inclusive) for user pools and in the range of 1 to 1000 (inclusive) for system pools. The default value is 1. | [optional]
**enable_auto_scaling** | Option<**bool**> | EnableAutoScaling - Whether to enable auto-scaler | [optional]
**mode** | Option<**String**> | Mode - Possible values include: 'System', 'User'. | [optional]
**orchestrator_version** | Option<**String**> | OrchestratorVersion - As a best practice, you should upgrade all node pools in an AKS cluster to the same Kubernetes version. The node pool version must have the same major version as the control plane. The node pool minor version must be within two minor versions of the control plane version. The node pool version cannot be greater than the control plane version. For more information see [upgrading a node pool](https://docs.microsoft.com/azure/aks/use-multiple-node-pools#upgrade-a-node-pool). | [optional]
**os_disk_size_gb** | Option<**i32**> | The OSDiskSize for Agent agentpool cannot be less than 30GB or larger than 2048GB. | [optional]
**scaling_config** | Option<[**crate::models::AksNodegroupScalingConfig**](AKSNodegroupScalingConfig.md)> |  | [optional]
**vm_size** | Option<**String**> | Required: VMSize - VM size availability varies by region. If a node contains insufficient compute resources (memory, cpu, etc) pods might fail to run correctly. For more details on restricted VM sizes, see: https://docs.microsoft.com/azure/aks/quotas-skus-regions | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


