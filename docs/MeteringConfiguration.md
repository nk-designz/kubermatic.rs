# MeteringConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> |  | [optional]
**reports** | Option<[**::std::collections::HashMap<String, crate::models::MeteringReportConfiguration>**](MeteringReportConfiguration.md)> | ReportConfigurations is a map of report configuration definitions. | [optional]
**storage_class_name** | Option<**String**> | StorageClassName is the name of the storage class that the metering prometheus instance uses to store metric data for reporting. | [optional]
**storage_size** | Option<**String**> | StorageSize is the size of the storage class. Default value is 100Gi. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


