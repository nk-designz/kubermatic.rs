# LoggingRateLimitSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ingestion_burst_size** | Option<**i32**> | IngestionBurstSize represents ingestion burst size in number of requests (nginx `burst`). | [optional]
**ingestion_rate** | Option<**i32**> | IngestionRate represents ingestion rate limit in requests per second (nginx `rate` in `r/s`). | [optional]
**query_burst_size** | Option<**i32**> | QueryBurstSize represents query burst size in number of requests (nginx `burst`). | [optional]
**query_rate** | Option<**i32**> | QueryRate represents query request rate limit per second (nginx `rate` in `r/s`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


