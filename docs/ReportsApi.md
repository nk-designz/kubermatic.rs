# \ReportsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_metering_reports**](ReportsApi.md#list_metering_reports) | **GET** /api/v1/admin/metering/reports | 



## list_metering_reports

> Vec<crate::models::MeteringReport> list_metering_reports(start_after, max_keys, configuration_name)


List metering reports. Only available in Kubermatic Enterprise Edition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_after** | Option<**String**> |  |  |
**max_keys** | Option<**i64**> |  |  |
**configuration_name** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::MeteringReport>**](MeteringReport.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

