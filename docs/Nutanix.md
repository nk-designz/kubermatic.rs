# Nutanix

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_name** | Option<**String**> | ClusterName is the Nutanix cluster to deploy resources and nodes to. | [optional]
**csi_endpoint** | Option<**String**> | CSIEndpoint to access Nutanix Prism Element for csi driver | [optional]
**csi_password** | Option<**String**> | Prism Element Password for csi driver | [optional]
**csi_port** | Option<**i32**> | CSIPort to use when connecting to the Nutanix Prism Element endpoint (defaults to 9440) | [optional]
**csi_username** | Option<**String**> | Prism Element Username for csi driver | [optional]
**datacenter** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**password** | Option<**String**> | Password is the password corresponding to the provided user. | [optional]
**project_name** | Option<**String**> | ProjectName is the optional Nutanix project to use. If none is given, no project will be used. | [optional]
**proxy_url** | Option<**String**> | ProxyURL is used to optionally configure a HTTP proxy to access Nutanix Prism Central. | [optional]
**username** | Option<**String**> | Username is the username to access the Nutanix Prism Central API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


