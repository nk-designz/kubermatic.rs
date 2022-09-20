# MonitoringRateLimitSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ingestion_burst_size** | Option<**i32**> | IngestionBurstSize represents ingestion burst size in samples per second (Cortex `ingestion_burst_size`). | [optional]
**ingestion_rate** | Option<**i32**> | IngestionRate represents the ingestion rate limit in samples per second (Cortex `ingestion_rate`). | [optional]
**max_samples_per_query** | Option<**i32**> | MaxSamplesPerQuery represents maximum number of samples during a query (Cortex `max_samples_per_query`). | [optional]
**max_series_per_metric** | Option<**i32**> | MaxSeriesPerMetric represents maximum number of series per metric (Cortex `max_series_per_metric`). | [optional]
**max_series_per_query** | Option<**i32**> | MaxSeriesPerQuery represents maximum number of timeseries during a query (Cortex `max_series_per_query`). | [optional]
**max_series_total** | Option<**i32**> | MaxSeriesTotal represents maximum number of series per this user cluster (Cortex `max_series_per_user`). | [optional]
**query_burst_size** | Option<**i32**> | QueryBurstSize represents query burst size in number of requests (nginx `burst`). | [optional]
**query_rate** | Option<**i32**> | QueryRate represents  query request rate limit per second (nginx `rate` in `r/s`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


