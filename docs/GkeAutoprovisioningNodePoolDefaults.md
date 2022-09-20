# GkeAutoprovisioningNodePoolDefaults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**boot_disk_kms_key** | Option<**String**> | BootDiskKmsKey: The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cr yptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption | [optional]
**disk_size_gb** | Option<**i64**> | DiskSizeGb: Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB. | [optional]
**disk_type** | Option<**String**> | DiskType: Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard' | [optional]
**management** | Option<[**crate::models::GkeNodeManagement**](GKENodeManagement.md)> |  | [optional]
**min_cpu_platform** | Option<**String**> | MinCpuPlatform: Minimum CPU platform to be used for NAP created node pools. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as minCpuPlatform: Intel Haswell or minCpuPlatform: Intel Sandy Bridge. For more information, read how to specify min CPU platform (https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform) To unset the min cpu platform field pass \"automatic\" as field value. | [optional]
**oauth_scopes** | Option<**Vec<String>**> | OauthScopes: Scopes that are used by NAP when creating node pools. | [optional]
**service_account** | Option<**String**> | ServiceAccount: The Google Cloud Platform Service Account to be used by the node VMs. | [optional]
**shielded_instance_config** | Option<[**crate::models::GkeShieldedInstanceConfig**](GKEShieldedInstanceConfig.md)> |  | [optional]
**upgrade_settings** | Option<[**crate::models::GkeUpgradeSettings**](GKEUpgradeSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


