# \PresetApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_preset**](PresetApi.md#create_preset) | **POST** /api/v2/providers/{provider_name}/presets | 
[**delete_preset**](PresetApi.md#delete_preset) | **DELETE** /api/v2/presets/{preset_name} | Removes preset.
[**delete_preset_provider**](PresetApi.md#delete_preset_provider) | **DELETE** /api/v2/presets/{preset_name}/provider/{provider_name} | Removes selected preset's provider.
[**delete_provider_preset**](PresetApi.md#delete_provider_preset) | **DELETE** /api/v2/providers/{provider_name}/presets/{preset_name} | Deletes provider preset.
[**get_preset_stats**](PresetApi.md#get_preset_stats) | **GET** /api/v2/presets/{preset_name}/stats | Gets presets stats.
[**list_presets**](PresetApi.md#list_presets) | **GET** /api/v2/presets | 
[**list_provider_presets**](PresetApi.md#list_provider_presets) | **GET** /api/v2/providers/{provider_name}/presets | 
[**update_preset**](PresetApi.md#update_preset) | **PUT** /api/v2/providers/{provider_name}/presets | 
[**update_preset_status**](PresetApi.md#update_preset_status) | **PUT** /api/v2/presets/{preset_name}/status | Updates the status of a preset. It can enable or disable it, so that it won't be listed by the list endpoints.



## create_preset

> crate::models::Preset create_preset(provider_name, body)


Creates the preset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**body** | [**PresetBody**](PresetBody.md) |  | [required] |

### Return type

[**crate::models::Preset**](Preset.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_preset

> delete_preset(preset_name)
Removes preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_preset_provider

> delete_preset_provider(preset_name, provider_name)
Removes selected preset's provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_name** | **String** |  | [required] |
**provider_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_provider_preset

> delete_provider_preset(provider_name, preset_name)
Deletes provider preset.

This endpoint has been depreciated in favour of /presets/{presets_name} and /presets/{preset_name}/providers/{provider_name}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**preset_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_preset_stats

> crate::models::PresetStats get_preset_stats(preset_name)
Gets presets stats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_name** | **String** |  | [required] |

### Return type

[**crate::models::PresetStats**](PresetStats.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_presets

> crate::models::PresetList list_presets(disabled)


Lists presets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disabled** | Option<**bool**> |  |  |

### Return type

[**crate::models::PresetList**](PresetList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_provider_presets

> crate::models::PresetList list_provider_presets(provider_name, disabled, datacenter)


Lists presets for the provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**disabled** | Option<**bool**> |  |  |
**datacenter** | Option<**String**> |  |  |

### Return type

[**crate::models::PresetList**](PresetList.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_preset

> crate::models::Preset update_preset(provider_name, body)


Updates provider preset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**body** | [**PresetBody**](PresetBody.md) |  | [required] |

### Return type

[**crate::models::Preset**](Preset.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_preset_status

> update_preset_status(preset_name, body, provider)
Updates the status of a preset. It can enable or disable it, so that it won't be listed by the list endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_name** | **String** |  | [required] |
**body** | [**PresetNameStatusBody**](PresetNameStatusBody.md) |  | [required] |
**provider** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

