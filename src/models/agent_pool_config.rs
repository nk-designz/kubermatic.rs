/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AgentPoolConfig {
    /// EnableNodePublicIP - Some scenarios may require nodes in a node pool to receive their own dedicated public IP addresses. A common scenario is for gaming workloads, where a console needs to make a direct connection to a cloud virtual machine to minimize hops. For more information see [assigning a public IP per node](https://docs.microsoft.com/azure/aks/use-multiple-node-pools#assign-a-public-ip-per-node-for-your-node-pools). The default is false.
    #[serde(rename = "enableNodePublicIP", skip_serializing_if = "Option::is_none")]
    pub enable_node_public_ip: Option<bool>,
    /// MaxPods - The maximum number of pods that can run on a node.
    #[serde(rename = "maxPods", skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,
    /// MaxSurgeUpgradeSetting - This can either be set to an integer (e.g. '5') or a percentage (e.g. '50%'). If a percentage is specified, it is the percentage of the total agent pool size at the time of the upgrade. For percentages, fractional nodes are rounded up. If not specified, the default is 1. For more information, including best practices, see: https://docs.microsoft.com/azure/aks/upgrade-cluster#customize-node-surge-upgrade
    #[serde(rename = "maxSurge", skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<String>,
    /// OsDiskType - Possible values include: 'Managed', 'Ephemeral'
    #[serde(rename = "osDiskType", skip_serializing_if = "Option::is_none")]
    pub os_disk_type: Option<String>,
    /// OsType - Possible values include: 'Linux', 'Windows'. The default value is 'Linux'. Windows node pools are not supported on kubenet clusters
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// PodSubnetID - If omitted, pod IPs are statically assigned on the node subnet (see vnetSubnetID for more details). This is of the form: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName}
    #[serde(rename = "podSubnetID", skip_serializing_if = "Option::is_none")]
    pub pod_subnet_id: Option<String>,
    /// VnetSubnetID - If this is not specified, a VNET and subnet will be generated and used. If no podSubnetID is specified, this applies to nodes and pods, otherwise it applies to just nodes. This is of the form: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName}
    #[serde(rename = "vnetSubnetID", skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<String>,
}

impl AgentPoolConfig {
    pub fn new() -> AgentPoolConfig {
        AgentPoolConfig {
            enable_node_public_ip: None,
            max_pods: None,
            max_surge: None,
            os_disk_type: None,
            os_type: None,
            pod_subnet_id: None,
            vnet_subnet_id: None,
        }
    }
}


