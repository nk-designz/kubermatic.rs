# AgentPoolConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_node_public_ip** | Option<**bool**> | EnableNodePublicIP - Some scenarios may require nodes in a node pool to receive their own dedicated public IP addresses. A common scenario is for gaming workloads, where a console needs to make a direct connection to a cloud virtual machine to minimize hops. For more information see [assigning a public IP per node](https://docs.microsoft.com/azure/aks/use-multiple-node-pools#assign-a-public-ip-per-node-for-your-node-pools). The default is false. | [optional]
**max_pods** | Option<**i32**> | MaxPods - The maximum number of pods that can run on a node. | [optional]
**max_surge** | Option<**String**> | MaxSurgeUpgradeSetting - This can either be set to an integer (e.g. '5') or a percentage (e.g. '50%'). If a percentage is specified, it is the percentage of the total agent pool size at the time of the upgrade. For percentages, fractional nodes are rounded up. If not specified, the default is 1. For more information, including best practices, see: https://docs.microsoft.com/azure/aks/upgrade-cluster#customize-node-surge-upgrade | [optional]
**os_disk_type** | Option<**String**> | OsDiskType - Possible values include: 'Managed', 'Ephemeral' | [optional]
**os_type** | Option<**String**> | OsType - Possible values include: 'Linux', 'Windows'. The default value is 'Linux'. Windows node pools are not supported on kubenet clusters | [optional]
**pod_subnet_id** | Option<**String**> | PodSubnetID - If omitted, pod IPs are statically assigned on the node subnet (see vnetSubnetID for more details). This is of the form: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName} | [optional]
**vnet_subnet_id** | Option<**String**> | VnetSubnetID - If this is not specified, a VNET and subnet will be generated and used. If no podSubnetID is specified, this applies to nodes and pods, otherwise it applies to just nodes. This is of the form: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName} | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


