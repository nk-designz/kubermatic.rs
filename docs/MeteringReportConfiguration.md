# MeteringReportConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | Option<**i32**> | Interval defines the number of days consulted in the metering report. | [optional]
**retention** | Option<**i32**> | Retention defines a number of days after which reports are queued for removal. If not set, reports are kept forever. Please note that this functionality works only for object storage that supports an object lifecycle management mechanism. | [optional]
**schedule** | Option<**String**> | Schedule in Cron format, see https://en.wikipedia.org/wiki/Cron. Please take a note that Schedule is responsible only for setting the time when a report generation mechanism kicks off. The Interval MUST be set independently. | [optional]
**_type** | Option<**Vec<String>**> | Types of reports to generate. Available report types are cluster and namespace. By default, all types of reports are generated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


