# ApplicationRef

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the Application. Should be a valid lowercase RFC1123 domain name | [optional]
**version** | Option<[**serde_json::Value**](.md)> | Version wraps semverlib.Version. It is needed because kubebuilder does not accept structs with non-tagged fields, even if they have custom marshallers With this the CRD resource will have Version as string but operator code can work directly with the semverlib.Version struct (taken from https://github.com/kubernetes-sigs/controller-tools/blob/master/pkg/crd/testdata/cronjob_types.go#L283) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


