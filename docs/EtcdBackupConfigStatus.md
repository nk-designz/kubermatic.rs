# EtcdBackupConfigStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cleanup_running** | Option<**bool**> | If the controller was configured with a cleanupContainer, CleanupRunning keeps track of the corresponding job | [optional]
**conditions** | Option<[**Vec<crate::models::EtcdBackupConfigCondition>**](EtcdBackupConfigCondition.md)> | Conditions contains conditions of the EtcdBackupConfig | [optional]
**last_backups** | Option<[**Vec<crate::models::BackupStatus>**](BackupStatus.md)> | CurrentBackups tracks the creation and deletion progress if all backups managed by the EtcdBackupConfig | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


