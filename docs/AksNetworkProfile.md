# AksNetworkProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dns_service_ip** | Option<**String**> | DNSServiceIP - An IP address assigned to the Kubernetes DNS service. It must be within the Kubernetes service address range specified in serviceCidr. | [optional]
**docker_bridge_cidr** | Option<**String**> | DockerBridgeCidr - A CIDR notation IP range assigned to the Docker bridge network. It must not overlap with any Subnet IP ranges or the Kubernetes service address range. | [optional]
**load_balancer_sku** | Option<**String**> | LoadBalancerSku - The default is 'standard'. See [Azure Load Balancer SKUs](https://docs.microsoft.com/azure/load-balancer/skus) for more information about the differences between load balancer SKUs. Possible values include: 'LoadBalancerSkuStandard', 'LoadBalancerSkuBasic' | [optional]
**network_mode** | Option<**String**> | NetworkMode - This cannot be specified if networkPlugin is anything other than 'azure'. Possible values include: 'Transparent', 'Bridge' | [optional]
**network_plugin** | Option<**String**> | NetworkPlugin - Network plugin used for building the Kubernetes network. Possible values include: 'Azure', 'Kubenet' | [optional]
**network_policy** | Option<**String**> | NetworkPolicy - Network policy used for building the Kubernetes network. Possible values include: 'Calico', 'Azure' | [optional]
**outbound_type** | Option<**String**> | OutboundType - This can only be set at cluster creation time and cannot be changed later. For more information see [egress outbound type](https://docs.microsoft.com/azure/aks/egress-outboundtype). Possible values include: 'OutboundTypeLoadBalancer', 'OutboundTypeUserDefinedRouting', 'OutboundTypeManagedNATGateway', 'OutboundTypeUserAssignedNATGateway' | [optional]
**pod_cidr** | Option<**String**> | PodCidr - A CIDR notation IP range from which to assign pod IPs when kubenet is used. | [optional]
**service_cidr** | Option<**String**> | ServiceCidr - A CIDR notation IP range from which to assign service cluster IPs. It must not overlap with any Subnet IP ranges. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


