# EtcdBackupConfigSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_id** | Option<**String**> | ClusterID is the id of the cluster which will be backed up | [optional]
**destination** | Option<**String**> | Destination indicates where the backup will be stored. The destination name should correspond to a destination in the cluster's Seed.Spec.EtcdBackupRestore. | [optional]
**keep** | Option<**i64**> | Keep is the number of backups to keep around before deleting the oldest one If not set, defaults to DefaultKeptBackupsCount. Only used if Schedule is set. | [optional]
**schedule** | Option<**String**> | Schedule is a cron expression defining when to perform the backup. If not set, the backup is performed exactly once, immediately. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


