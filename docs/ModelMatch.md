# ModelMatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**excluded_namespaces** | Option<**Vec<String>**> | ExcludedNamespaces is a list of namespace names. If defined, a constraint will only apply to resources not in a listed namespace. | [optional]
**kinds** | Option<[**Vec<crate::models::Kind>**](Kind.md)> | Kinds accepts a list of objects with apiGroups and kinds fields that list the groups/kinds of objects to which the constraint will apply. If multiple groups/kinds objects are specified, only one match is needed for the resource to be in scope | [optional]
**label_selector** | Option<[**crate::models::LabelSelector**](LabelSelector.md)> |  | [optional]
**namespace_selector** | Option<[**crate::models::LabelSelector**](LabelSelector.md)> |  | [optional]
**namespaces** | Option<**Vec<String>**> | Namespaces is a list of namespace names. If defined, a constraint will only apply to resources in a listed namespace. | [optional]
**scope** | Option<**String**> | Scope accepts *, Cluster, or Namespaced which determines if cluster-scoped and/or namesapced-scoped resources are selected. (defaults to *) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


