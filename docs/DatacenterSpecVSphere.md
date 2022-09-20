# DatacenterSpecVSphere

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_insecure** | Option<**bool**> | If set to true, disables the TLS certificate check against the endpoint. | [optional]
**cluster** | Option<**String**> | The name of the vSphere cluster to use. Used for out-of-tree CSI Driver. | [optional]
**datacenter** | Option<**String**> | The name of the datacenter to use. | [optional]
**datastore** | Option<**String**> | The default Datastore to be used for provisioning volumes using storage classes/dynamic provisioning and for storing virtual machine files in case no `Datastore` or `DatastoreCluster` is provided at Cluster level. | [optional]
**endpoint** | Option<**String**> | Endpoint URL to use, including protocol, for example \"https://vcenter.example.com\". | [optional]
**infra_management_user** | Option<[**crate::models::VSphereCredentials**](VSphereCredentials.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | Optional: defines if the IPv6 is enabled for the datacenter | [optional]
**root_path** | Option<**String**> | Optional: The root path for cluster specific VM folders. Each cluster gets its own folder below the root folder. Must be the FQDN (for example \"/datacenter-1/vm/all-kubermatic-vms-in-here\") and defaults to the root VM folder: \"/datacenter-1/vm\" | [optional]
**storage_policy** | Option<**String**> | The name of the storage policy to use for the storage class created in the user cluster. | [optional]
**templates** | Option<[**crate::models::ImageList**](ImageList.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


