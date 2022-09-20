# NetworkDefaultsIpFamily

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_cidr_mask_size** | Option<**i32**> | NodeCIDRMaskSize contains the default mask size used to address the nodes within provided Pods CIDR. | [optional]
**node_ports_allowed_ip_range** | Option<**String**> | NodePortsAllowedIPRange defines the default IP range from which access to NodePort services is allowed for applicable cloud providers. | [optional]
**pods_cidr** | Option<**String**> | PodsCIDR contains the default network range from which POD networks are allocated. | [optional]
**services_cidr** | Option<**String**> | ServicesCIDR contains the default network range from which service VIPs are allocated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


