# GitCredentials

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method** | Option<**String**> | +kubebuilder:validation:Enum=password;token;ssh-key | [optional]
**password** | Option<[**crate::models::SecretKeySelector**](SecretKeySelector.md)> |  | [optional]
**ssh_key** | Option<[**crate::models::SecretKeySelector**](SecretKeySelector.md)> |  | [optional]
**token** | Option<[**crate::models::SecretKeySelector**](SecretKeySelector.md)> |  | [optional]
**username** | Option<[**crate::models::SecretKeySelector**](SecretKeySelector.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


