# EtcdBackupRestore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_destination** | Option<**String**> | DefaultDestination marks the default destination that will be used for the default etcd backup config which is created for every user cluster. Has to correspond to a destination in Destinations. If removed, it removes the related default etcd backup configs. | [optional]
**destinations** | Option<[**::std::collections::HashMap<String, crate::models::BackupDestination>**](BackupDestination.md)> | Destinations stores all the possible destinations where the backups for the Seed can be stored. If not empty, it enables automatic backup and restore for the seed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


