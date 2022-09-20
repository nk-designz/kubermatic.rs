# HetznerCloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credentials_reference** | Option<[**crate::models::GlobalObjectKeySelector**](GlobalObjectKeySelector.md)> |  | [optional]
**network** | Option<**String**> | Network is the pre-existing Hetzner network in which the machines are running. While machines can be in multiple networks, a single one must be chosen for the HCloud CCM to work. If this is empty, the network configured on the datacenter will be used. | [optional]
**token** | Option<**String**> | Token is used to authenticate with the Hetzner cloud API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


