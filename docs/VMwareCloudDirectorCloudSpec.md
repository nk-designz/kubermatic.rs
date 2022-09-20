# VMwareCloudDirectorCloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credentials_reference** | Option<[**crate::models::GlobalObjectKeySelector**](GlobalObjectKeySelector.md)> |  | [optional]
**csi** | Option<[**crate::models::VMwareCloudDirectorCsiConfig**](VMwareCloudDirectorCSIConfig.md)> |  | [optional]
**organization** | Option<**String**> | Organization is the name of organization to use. +optional | [optional]
**ovdc_network** | Option<**String**> | Network is the name of organizational virtual data center network that will be associated with the VMs and vApp. | [optional]
**password** | Option<**String**> | Password is the VMware Cloud Director user password. +optional | [optional]
**username** | Option<**String**> | Username is the VMware Cloud Director user name. +optional | [optional]
**vapp** | Option<**String**> | VApp used for isolation of VMs and their associated network +optional | [optional]
**vdc** | Option<**String**> | VDC is the organizational virtual data center. +optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


