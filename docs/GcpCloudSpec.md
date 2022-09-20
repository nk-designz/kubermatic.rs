# GcpCloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credentials_reference** | Option<[**crate::models::GlobalObjectKeySelector**](GlobalObjectKeySelector.md)> |  | [optional]
**network** | Option<**String**> |  | [optional]
**node_ports_allowed_ip_range** | Option<**String**> | A CIDR range that will be used to allow access to the node port range in the firewall rules to. If NodePortsAllowedIPRange nor NodePortsAllowedIPRanges is set, the node port range can be accessed from anywhere. | [optional]
**node_ports_allowed_ip_ranges** | Option<[**crate::models::NetworkRanges**](NetworkRanges.md)> |  | [optional]
**service_account** | Option<**String**> | The Google Service Account (JSON format), encoded with base64. | [optional]
**subnetwork** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


