# OpaIntegrationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audit_resources** | Option<[**crate::models::ResourceRequirements**](ResourceRequirements.md)> |  | [optional]
**controller_resources** | Option<[**crate::models::ResourceRequirements**](ResourceRequirements.md)> |  | [optional]
**enabled** | Option<**bool**> | Enables OPA Gatekeeper integration. | [optional]
**experimental_enable_mutation** | Option<**bool**> | Optional: Enables experimental mutation in Gatekeeper. | [optional]
**webhook_timeout_seconds** | Option<**i32**> | The timeout in seconds that is set for the Gatekeeper validating webhook admission review calls. Defaults to `10` (seconds). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


