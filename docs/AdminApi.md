# \AdminApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_metering_report_configuration**](AdminApi.md#create_metering_report_configuration) | **POST** /api/v1/admin/metering/configurations/reports/{name} | 
[**create_or_update_metering_configurations**](AdminApi.md#create_or_update_metering_configurations) | **PUT** /api/v1/admin/metering/configurations | 
[**create_or_update_metering_credentials**](AdminApi.md#create_or_update_metering_credentials) | **PUT** /api/v1/admin/metering/credentials | 
[**create_resource_quota**](AdminApi.md#create_resource_quota) | **POST** /api/v2/quotas | Creates a new Resource Quota.
[**create_seed**](AdminApi.md#create_seed) | **POST** /api/v1/admin/seeds | Creates a new seed object.
[**delete_admission_plugin**](AdminApi.md#delete_admission_plugin) | **DELETE** /api/v1/admin/admission/plugins/{name} | Deletes the admission plugin.
[**delete_backup_destination**](AdminApi.md#delete_backup_destination) | **DELETE** /api/v1/admin/seeds/{seed_name}/backupdestinations/{backup_destination} | Deletes a backup destination from the Seed.
[**delete_metering_report_configuration**](AdminApi.md#delete_metering_report_configuration) | **DELETE** /api/v1/admin/metering/configurations/reports/{name} | 
[**delete_resource_quota**](AdminApi.md#delete_resource_quota) | **DELETE** /api/v2/quotas/{quota_name} | Removes an existing Resource Quota.
[**delete_seed**](AdminApi.md#delete_seed) | **DELETE** /api/v1/admin/seeds/{seed_name} | Deletes the seed CRD object from the Kubermatic.
[**get_admins**](AdminApi.md#get_admins) | **GET** /api/v1/admin | Returns list of admin users.
[**get_admission_plugin**](AdminApi.md#get_admission_plugin) | **GET** /api/v1/admin/admission/plugins/{name} | Gets the admission plugin.
[**get_kubermatic_custom_links**](AdminApi.md#get_kubermatic_custom_links) | **GET** /api/v1/admin/settings/customlinks | Gets the custom links.
[**get_kubermatic_settings**](AdminApi.md#get_kubermatic_settings) | **GET** /api/v1/admin/settings | Gets the global settings.
[**get_metering_report_configuration**](AdminApi.md#get_metering_report_configuration) | **GET** /api/v1/admin/metering/configurations/reports/{name} | 
[**get_resource_quota**](AdminApi.md#get_resource_quota) | **GET** /api/v2/quotas/{quota_name} | Gets a specific Resource Quota.
[**get_seed**](AdminApi.md#get_seed) | **GET** /api/v1/admin/seeds/{seed_name} | Returns the seed object.
[**list_admission_plugins**](AdminApi.md#list_admission_plugins) | **GET** /api/v1/admin/admission/plugins | Returns all admission plugins from the CRDs.
[**list_metering_report_configurations**](AdminApi.md#list_metering_report_configurations) | **GET** /api/v1/admin/metering/configurations/reports | 
[**list_resource_quotas**](AdminApi.md#list_resource_quotas) | **GET** /api/v2/quotas | Gets a Resource Quota list.
[**list_seeds**](AdminApi.md#list_seeds) | **GET** /api/v1/admin/seeds | Returns all seeds from the CRDs.
[**patch_kubermatic_settings**](AdminApi.md#patch_kubermatic_settings) | **PATCH** /api/v1/admin/settings | Patches the global settings.
[**put_resource_quota**](AdminApi.md#put_resource_quota) | **PUT** /api/v2/quotas/{quota_name} | Updates an existing Resource Quota.
[**set_admin**](AdminApi.md#set_admin) | **PUT** /api/v1/admin | Allows setting and clearing admin role for users.
[**update_admission_plugin**](AdminApi.md#update_admission_plugin) | **PATCH** /api/v1/admin/admission/plugins/{name} | Updates the admission plugin.
[**update_metering_report_configuration**](AdminApi.md#update_metering_report_configuration) | **PUT** /api/v1/admin/metering/configurations/reports/{name} | 
[**update_seed**](AdminApi.md#update_seed) | **PATCH** /api/v1/admin/seeds/{seed_name} | Updates the seed.



## create_metering_report_configuration

> create_metering_report_configuration(name, body)


Creates report configuration for KKP metering tool. Only available in Kubermatic Enterprise Edition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**body** | Option<[**InlineObject1**](InlineObject1.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_metering_configurations

> create_or_update_metering_configurations()


Configures KKP metering tool. Only available in Kubermatic Enterprise Edition

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_metering_credentials

> create_or_update_metering_credentials()


Creates or updates the metering tool credentials. Only available in Kubermatic Enterprise Edition

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_resource_quota

> create_resource_quota(body)
Creates a new Resource Quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InlineObject13**](InlineObject13.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_seed

> crate::models::Seed create_seed(body)
Creates a new seed object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**InlineObject2**](InlineObject2.md)> |  |  |

### Return type

[**crate::models::Seed**](Seed.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_admission_plugin

> delete_admission_plugin(name)
Deletes the admission plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_backup_destination

> delete_backup_destination(seed_name, backup_destination)
Deletes a backup destination from the Seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**backup_destination** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_metering_report_configuration

> delete_metering_report_configuration(name)


Removes report configuration for KKP metering tool. Only available in Kubermatic Enterprise Edition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_resource_quota

> delete_resource_quota(quota_name)
Removes an existing Resource Quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_seed

> delete_seed(seed_name)
Deletes the seed CRD object from the Kubermatic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_admins

> Vec<crate::models::Admin> get_admins()
Returns list of admin users.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Admin>**](Admin.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_admission_plugin

> crate::models::AdmissionPlugin get_admission_plugin(name)
Gets the admission plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::AdmissionPlugin**](AdmissionPlugin.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kubermatic_custom_links

> Vec<crate::models::CustomLink> get_kubermatic_custom_links()
Gets the custom links.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CustomLink>**](CustomLink.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kubermatic_settings

> crate::models::SettingSpec get_kubermatic_settings()
Gets the global settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SettingSpec**](SettingSpec.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_report_configuration

> crate::models::MeteringReportConfiguration get_metering_report_configuration(name)


Gets report configuration for KKP metering tool. Only available in Kubermatic Enterprise Edition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::MeteringReportConfiguration**](MeteringReportConfiguration.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_quota

> crate::models::ResourceQuota get_resource_quota(quota_name)
Gets a specific Resource Quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_name** | **String** |  | [required] |

### Return type

[**crate::models::ResourceQuota**](ResourceQuota.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_seed

> crate::models::Seed get_seed(seed_name)
Returns the seed object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |

### Return type

[**crate::models::Seed**](Seed.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_admission_plugins

> Vec<crate::models::AdmissionPlugin> list_admission_plugins()
Returns all admission plugins from the CRDs.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AdmissionPlugin>**](AdmissionPlugin.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metering_report_configurations

> Vec<crate::models::MeteringReportConfiguration> list_metering_report_configurations()


Lists report configurations for KKP metering tool. Only available in Kubermatic Enterprise Edition

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MeteringReportConfiguration>**](MeteringReportConfiguration.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_resource_quotas

> Vec<crate::models::ResourceQuota> list_resource_quotas(subject_name, subject_kind)
Gets a Resource Quota list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_name** | Option<**String**> |  |  |
**subject_kind** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ResourceQuota>**](ResourceQuota.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_seeds

> Vec<crate::models::Seed> list_seeds()
Returns all seeds from the CRDs.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Seed>**](Seed.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_kubermatic_settings

> crate::models::SettingSpec patch_kubermatic_settings(patch)
Patches the global settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::SettingSpec**](SettingSpec.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_resource_quota

> put_resource_quota(quota_name, body)
Updates an existing Resource Quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_name** | **String** |  | [required] |
**body** | [**Quota**](Quota.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_admin

> crate::models::Admin set_admin(body)
Allows setting and clearing admin role for users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Admin**](Admin.md)> |  |  |

### Return type

[**crate::models::Admin**](Admin.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_admission_plugin

> crate::models::AdmissionPlugin update_admission_plugin(name, body)
Updates the admission plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**body** | Option<[**AdmissionPlugin**](AdmissionPlugin.md)> |  |  |

### Return type

[**crate::models::AdmissionPlugin**](AdmissionPlugin.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metering_report_configuration

> update_metering_report_configuration(name, body)


Updates existing report configuration for KKP metering tool. Only available in Kubermatic Enterprise Edition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**body** | Option<[**InlineObject**](InlineObject.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_seed

> crate::models::Seed update_seed(seed_name, body)
Updates the seed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seed_name** | **String** |  | [required] |
**body** | Option<[**InlineObject3**](InlineObject3.md)> |  |  |

### Return type

[**crate::models::Seed**](Seed.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

