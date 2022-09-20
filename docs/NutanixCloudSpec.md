# NutanixCloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_name** | Option<**String**> | ClusterName is the Nutanix cluster that this user cluster will be deployed to. | [optional]
**credentials_reference** | Option<[**crate::models::GlobalObjectKeySelector**](GlobalObjectKeySelector.md)> |  | [optional]
**csi** | Option<[**crate::models::NutanixCsiConfig**](NutanixCSIConfig.md)> |  | [optional]
**password** | Option<**String**> |  | [optional]
**project_name** | Option<**String**> | ProjectName is the project that this cluster is deployed into. If none is given, no project will be used. +optional | [optional]
**proxy_url** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


