# KubevirtCloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credentials_reference** | Option<[**crate::models::GlobalObjectKeySelector**](GlobalObjectKeySelector.md)> |  | [optional]
**csi_kubeconfig** | Option<**String**> |  | [optional]
**infra_storage_classes** | Option<**Vec<String>**> | InfraStorageClasses is a list of storage classes from KubeVirt infra cluster that are used for initialization of user cluster storage classes by the CSI driver kubevirt (hot pluggable disks) | [optional]
**kubeconfig** | Option<**String**> | The cluster's kubeconfig file, encoded with base64. | [optional]
**pre_allocated_data_volumes** | Option<[**Vec<crate::models::PreAllocatedDataVolume>**](PreAllocatedDataVolume.md)> | PreAllocatedDataVolumes holds list of preallocated DataVolumes which can be used as reference for DataVolume cloning. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


