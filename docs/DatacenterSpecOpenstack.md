# DatacenterSpecOpenstack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_url** | Option<**String**> |  | [optional]
**availability_zone** | Option<**String**> |  | [optional]
**dns_servers** | Option<**Vec<String>**> | Used for automatic network creation | [optional]
**enabled_flavors** | Option<**Vec<String>**> | Optional: List of enabled flavors for the given datacenter | [optional]
**enforce_floating_ip** | Option<**bool**> | Optional | [optional]
**ignore_volume_az** | Option<**bool**> | Optional | [optional]
**images** | Option<[**crate::models::ImageList**](ImageList.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | Optional: defines if the IPv6 is enabled for the datacenter | [optional]
**manage_security_groups** | Option<**bool**> | Optional: Gets mapped to the \"manage-security-groups\" setting in the cloud config. See https://kubernetes.io/docs/concepts/cluster-administration/cloud-providers/#load-balancer This setting defaults to true. | [optional]
**node_size_requirements** | Option<[**crate::models::OpenstackNodeSizeRequirements**](OpenstackNodeSizeRequirements.md)> |  | [optional]
**region** | Option<**String**> |  | [optional]
**trust_device_path** | Option<**bool**> | Optional: Gets mapped to the \"trust-device-path\" setting in the cloud config. See https://kubernetes.io/docs/concepts/cluster-administration/cloud-providers/#block-storage This setting defaults to false. | [optional]
**use_octavia** | Option<**bool**> | Optional: Gets mapped to the \"use-octavia\" setting in the cloud config. use-octavia is enabled by default in CCM since v1.17.0, and disabled by default with the in-tree cloud provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


