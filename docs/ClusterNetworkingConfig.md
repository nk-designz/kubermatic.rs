# ClusterNetworkingConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**core_dns_replicas** | Option<**i32**> | CoreDNSReplicas is the number of desired pods of user cluster coredns deployment. | [optional]
**dns_domain** | Option<**String**> | Domain name for services. | [optional]
**ip_family** | Option<**String**> | +kubebuilder:validation:Enum=\"\";IPv4;IPv4+IPv6 | [optional]
**ipvs** | Option<[**crate::models::IpvsConfiguration**](IPVSConfiguration.md)> |  | [optional]
**konnectivity_enabled** | Option<**bool**> | KonnectivityEnabled enables konnectivity for controlplane to node network communication. | [optional]
**node_cidr_mask_size_ipv4** | Option<**i32**> | NodeCIDRMaskSizeIPv4 is the mask size used to address the nodes within provided IPv4 Pods CIDR. It has to be larger than the provided IPv4 Pods CIDR. Defaults to 24. +optional | [optional]
**node_cidr_mask_size_ipv6** | Option<**i32**> | NodeCIDRMaskSizeIPv6 is the mask size used to address the nodes within provided IPv6 Pods CIDR. It has to be larger than the provided IPv6 Pods CIDR. Defaults to 64. +optional | [optional]
**node_local_dns_cache_enabled** | Option<**bool**> | NodeLocalDNSCacheEnabled controls whether the NodeLocal DNS Cache feature is enabled. Defaults to true. | [optional]
**pods** | Option<[**crate::models::NetworkRanges**](NetworkRanges.md)> |  | [optional]
**proxy_mode** | Option<**String**> | ProxyMode defines the kube-proxy mode (\"ipvs\" / \"iptables\" / \"ebpf\"). Defaults to \"ipvs\". \"ebpf\" disables kube-proxy and requires CNI support. | [optional]
**services** | Option<[**crate::models::NetworkRanges**](NetworkRanges.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


