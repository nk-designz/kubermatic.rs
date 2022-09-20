# GitSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credentials** | Option<[**crate::models::GitCredentials**](GitCredentials.md)> |  | [optional]
**path** | Option<**String**> | Path of the \"source\" in the repository. default is repository root | [optional]
**_ref** | Option<[**crate::models::GitReference**](GitReference.md)> |  | [optional]
**remote** | Option<**String**> | URL to the repository. Can be HTTP(s) (e.g. https://example.com/myrepo) or SSH (e.g. git://example.com[:port]/path/to/repo.git/) +kubebuilder:validation:MinLength=1 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


