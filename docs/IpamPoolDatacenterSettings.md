# IpamPoolDatacenterSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allocation_prefix** | Option<**i64**> |  | [optional]
**allocation_range** | Option<**i64**> |  | [optional]
**pool_cidr** | Option<**String**> | SubnetCIDR is used to store IPv4/IPv6 CIDR. | [optional]
**_type** | Option<**String**> | +kubebuilder:validation:Enum=prefix;range IPAMPoolAllocationType defines the type of allocation to be used. Possible values are `prefix` and `range`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


