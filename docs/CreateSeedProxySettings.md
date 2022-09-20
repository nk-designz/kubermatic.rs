# CreateSeedProxySettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**http_proxy** | Option<**String**> | Optional: If set, this proxy will be configured for both HTTP and HTTPS. | [optional]
**no_proxy** | Option<**String**> | Optional: If set this will be set as NO_PROXY environment variable on the node; The value must be a comma-separated list of domains for which no proxy should be used, e.g. \"*.example.com,internal.dev\". Note that the in-cluster apiserver URL will be automatically prepended to this value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


