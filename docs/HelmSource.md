# HelmSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chart_name** | Option<**String**> | Name of the Chart. +kubebuilder:validation:MinLength=1 | [optional]
**chart_version** | Option<**String**> | Version of the Chart. +kubebuilder:validation:MinLength=1 | [optional]
**credentials** | Option<[**crate::models::HelmCredentials**](HelmCredentials.md)> |  | [optional]
**url** | Option<**String**> | URl of the helm repository. It can be an HTTP(s) repository (e.g. https://localhost/myrepo) or on OCI repository (e.g. oci://localhost:5000/myrepo). +kubebuilder:validation:Pattern=\"^(http|https|oci)://.+\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


