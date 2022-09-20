# \MeteringApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_metering_report**](MeteringApi.md#delete_metering_report) | **DELETE** /api/v1/admin/metering/reports/{report_name} | 
[**get_metering_report**](MeteringApi.md#get_metering_report) | **GET** /api/v1/admin/metering/reports/{report_name} | 
[**list_metering_reports**](MeteringApi.md#list_metering_reports) | **GET** /api/v1/admin/metering/reports | 



## delete_metering_report

> delete_metering_report(report_name, configuration_name)


Removes a specific metering report. Only available in Kubermatic Enterprise Edition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_name** | **String** |  | [required] |
**configuration_name** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_report

> String get_metering_report(report_name, configuration_name)


Download a specific metering report. Provides an S3 pre signed URL valid for 1 hour. Only available in Kubermatic Enterprise Edition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_name** | **String** |  | [required] |
**configuration_name** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

