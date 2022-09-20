# KubevirtNodeSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpus** | **String** | CPUs states how many cpus the kubevirt node will have. | 
**flavor_name** | Option<**String**> | FlavorName states name of the virtual-machine flavor. | [optional]
**flavor_profile** | Option<**String**> | FlavorProfile states name of virtual-machine profile. | [optional]
**memory** | **String** | Memory states the memory that kubevirt node will have. | 
**node_affinity_preset** | Option<[**crate::models::NodeAffinityPreset**](NodeAffinityPreset.md)> |  | [optional]
**pod_affinity_preset** | Option<**String**> | PodAffinityPreset describes pod affinity scheduling rules | [optional]
**pod_anti_affinity_preset** | Option<**String**> | PodAntiAffinityPreset describes pod anti-affinity scheduling rules | [optional]
**primary_disk_os_image** | **String** | PrimaryDiskOSImage states the source from which the imported image will be downloaded. This field contains: a URL to download an Os Image from a HTTP source. a DataVolume Name as source for DataVolume cloning. | 
**primary_disk_size** | **String** | PrimaryDiskSize states the size of the provisioned pvc per node. | 
**primary_disk_storage_class_name** | **String** | PrimaryDiskStorageClassName states the storage class name for the provisioned PVCs. | 
**secondary_disks** | Option<[**Vec<crate::models::SecondaryDisks>**](SecondaryDisks.md)> | SecondaryDisks contains list of secondary-disks | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


