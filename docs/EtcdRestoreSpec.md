# EtcdRestoreSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_download_credentials_secret** | Option<**String**> | BackupDownloadCredentialsSecret is the name of a secret in the cluster-xxx namespace containing credentials needed to download the backup | [optional]
**backup_name** | Option<**String**> | BackupName is the name of the backup to restore from | [optional]
**cluster_id** | Option<**String**> | ClusterID is the id of the cluster which will be restored from the backup | [optional]
**destination** | Option<**String**> | Destination indicates where the backup was stored. The destination name should correspond to a destination in the cluster's Seed.Spec.EtcdBackupRestore. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


