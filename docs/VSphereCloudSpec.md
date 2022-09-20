# VSphereCloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credentials_reference** | Option<[**crate::models::GlobalObjectKeySelector**](GlobalObjectKeySelector.md)> |  | [optional]
**datastore** | Option<**String**> | Datastore to be used for storing virtual machines and as a default for dynamic volume provisioning, it is mutually exclusive with DatastoreCluster. +optional | [optional]
**datastore_cluster** | Option<**String**> | DatastoreCluster to be used for storing virtual machines, it is mutually exclusive with Datastore. +optional | [optional]
**folder** | Option<**String**> | Folder is the folder to be used to group the provisioned virtual machines. +optional | [optional]
**infra_management_user** | Option<[**crate::models::VSphereCredentials**](VSphereCredentials.md)> |  | [optional]
**password** | Option<**String**> | Password is the vSphere user password. +optional | [optional]
**resource_pool** | Option<**String**> | ResourcePool is used to manage resources such as cpu and memory for vSphere virtual machines. The resource pool should be defined on vSphere cluster level. +optional | [optional]
**storage_policy** | Option<**String**> | StoragePolicy to be used for storage provisioning | [optional]
**tag_category_id** | Option<**String**> | This is category for the machine deployment tags | [optional]
**username** | Option<**String**> | Username is the vSphere user name. +optional | [optional]
**vm_net_name** | Option<**String**> | VMNetName is the name of the vSphere network. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


