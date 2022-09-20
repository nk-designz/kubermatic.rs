# Subject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_group** | Option<**String**> | APIGroup holds the API group of the referenced subject. Defaults to \"\" for ServiceAccount subjects. Defaults to \"rbac.authorization.k8s.io\" for User and Group subjects. +optional | [optional]
**kind** | Option<**String**> | Kind of object being referenced. Values defined by this API group are \"User\", \"Group\", and \"ServiceAccount\". If the Authorizer does not recognized the kind value, the Authorizer should report an error. | [optional]
**name** | Option<**String**> | Name of the object being referenced. | [optional]
**namespace** | Option<**String**> | Namespace of the referenced object.  If the object kind is non-namespace, such as \"User\" or \"Group\", and this value is not empty the Authorizer should report an error. +optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


