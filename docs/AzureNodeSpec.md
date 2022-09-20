# AzureNodeSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assign_availability_set** | Option<**bool**> | AssignAvailabilitySet is used to check if an availability set should be created and assigned to the cluster. | [optional]
**assign_public_ip** | Option<**bool**> | should the machine have a publicly accessible IP address | [optional]
**data_disk_size** | Option<**i32**> | Data disk size in GB | [optional]
**image_id** | Option<**String**> | ImageID represents the ID of the image that should be used to run the node | [optional]
**os_disk_size** | Option<**i32**> | OS disk size in GB | [optional]
**size** | **String** | VM size | 
**tags** | Option<**::std::collections::HashMap<String, String>**> | Additional metadata to set | [optional]
**zones** | Option<**Vec<String>**> | Zones represents the availability zones for azure vms | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


