# OpenstackCloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_credential_id** | Option<**String**> |  | [optional]
**application_credential_secret** | Option<**String**> |  | [optional]
**credentials_reference** | Option<[**crate::models::GlobalObjectKeySelector**](GlobalObjectKeySelector.md)> |  | [optional]
**domain** | Option<**String**> |  | [optional]
**enable_ingress_hostname** | Option<**bool**> | Enable the `enable-ingress-hostname` cloud provider option on the Openstack CCM. Can only be used with the external CCM and might be deprecated and removed in future versions as it is considered a workaround for the PROXY protocol to preserve client IPs. +optional | [optional]
**floating_ip_pool** | Option<**String**> | FloatingIPPool holds the name of the public network The public network is reachable from the outside world and should provide the pool of IP addresses to choose from.  When specified, all worker nodes will receive a public ip from this floating ip pool  Note that the network is external if the \"External\" field is set to true | [optional]
**ingress_hostname_suffix** | Option<**String**> | Set a specific suffix for the hostnames used for the PROXY protocol workaround that is enabled by EnableIngressHostname. The suffix is set to `nip.io` by default. Can only be used with the external CCM and might be deprecated and removed in future versions as it is considered a workaround only. | [optional]
**ipv6_subnet_id** | Option<**String**> | IPv6SubnetID holds the ID of the subnet used for IPv6 networking. If not provided, a new subnet will be created if IPv6 is enabled. +optional | [optional]
**ipv6_subnet_pool** | Option<**String**> | IPv6SubnetPool holds the name of the subnet pool used for creating new IPv6 subnets. If not provided, the default IPv6 subnet pool will be used. +optional | [optional]
**network** | Option<**String**> | Network holds the name of the internal network When specified, all worker nodes will be attached to this network. If not specified, a network, subnet & router will be created  Note that the network is internal if the \"External\" field is set to false | [optional]
**node_ports_allowed_ip_range** | Option<**String**> | A CIDR range that will be used to allow access to the node port range in the security group to. Only applies if the security group is generated by KKP and not preexisting. If NodePortsAllowedIPRange nor NodePortsAllowedIPRanges is set, the node port range can be accessed from anywhere. | [optional]
**node_ports_allowed_ip_ranges** | Option<[**crate::models::NetworkRanges**](NetworkRanges.md)> |  | [optional]
**password** | Option<**String**> |  | [optional]
**project** | Option<**String**> | project, formally known as tenant. | [optional]
**project_id** | Option<**String**> | project id, formally known as tenantID. | [optional]
**router_id** | Option<**String**> |  | [optional]
**security_groups** | Option<**String**> |  | [optional]
**subnet_id** | Option<**String**> |  | [optional]
**token** | Option<**String**> | Used internally during cluster creation | [optional]
**use_octavia** | Option<**bool**> | Whether or not to use Octavia for LoadBalancer type of Service implementation instead of using Neutron-LBaaS. Attention:Openstack CCM use Octavia as default load balancer implementation since v1.17.0  Takes precedence over the 'use_octavia' flag provided at datacenter level if both are specified. +optional | [optional]
**use_token** | Option<**bool**> |  | [optional]
**username** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


