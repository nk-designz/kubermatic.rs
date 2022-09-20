# PolicyRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_groups** | Option<**Vec<String>**> | APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed. +optional | [optional]
**non_resource_urls** | Option<**Vec<String>**> | NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as \"pods\" or \"secrets\") or non-resource URL paths (such as \"/api\"),  but not both. +optional | [optional]
**resource_names** | Option<**Vec<String>**> | ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed. +optional | [optional]
**resources** | Option<**Vec<String>**> | Resources is a list of resources this rule applies to. '*' represents all resources. +optional | [optional]
**verbs** | Option<**Vec<String>**> | Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule. '*' represents all verbs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


