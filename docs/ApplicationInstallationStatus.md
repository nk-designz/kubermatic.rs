# ApplicationInstallationStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_version** | Option<[**crate::models::ApplicationVersion**](ApplicationVersion.md)> |  | [optional]
**conditions** | Option<[**Vec<crate::models::ApplicationInstallationCondition>**](ApplicationInstallationCondition.md)> | Conditions contains conditions an installation is in, its primary use case is status signaling between controllers or between controllers and the API | [optional]
**method** | Option<**String**> | +kubebuilder:validation:Enum=helm | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


