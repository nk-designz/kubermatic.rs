# NodeSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud** | [**crate::models::NodeCloudSpec**](NodeCloudSpec.md) |  | 
**labels** | Option<**::std::collections::HashMap<String, String>**> | Map of string keys and values that can be used to organize and categorize (scope and select) objects. It will be applied to Nodes allowing users run their apps on specific Node using labelSelector. | [optional]
**operating_system** | [**crate::models::OperatingSystemSpec**](OperatingSystemSpec.md) |  | 
**ssh_user_name** | Option<**String**> |  | [optional]
**taints** | Option<[**Vec<crate::models::TaintSpec>**](TaintSpec.md)> | List of taints to set on new nodes | [optional]
**versions** | [**crate::models::NodeVersionInfo**](NodeVersionInfo.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


