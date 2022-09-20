# GitReference

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**branch** | Option<**String**> | Branch to checkout. Only the last commit of the branch will be checkout in order to reduce the amount of data to download. +optional | [optional]
**commit** | Option<**String**> | Commit SHA in a Branch to checkout.  It must be used in conjunction with branch field. | [optional]
**tag** | Option<**String**> | Tag to check out. It can not be used in conjunction with commit or branch. +kubebuilder:validation:Type=string +optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


