# ApplicationInstallationSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_ref** | Option<[**crate::models::ApplicationRef**](ApplicationRef.md)> |  | [optional]
**namespace** | Option<[**crate::models::NamespaceSpec**](NamespaceSpec.md)> |  | [optional]
**values** | Option<[**serde_json::Value**](.md)> | To use this, make a field which has RawExtension as its type in your external, versioned struct, and Object in your internal struct. You also need to register your various plugin types.  Internal package: type MyAPIObject struct { runtime.TypeMeta `json:\",inline\"` MyPlugin runtime.Object `json:\"myPlugin\"` } type PluginA struct { AOption string `json:\"aOption\"` }  External package: type MyAPIObject struct { runtime.TypeMeta `json:\",inline\"` MyPlugin runtime.RawExtension `json:\"myPlugin\"` } type PluginA struct { AOption string `json:\"aOption\"` }  On the wire, the JSON will look something like this: { \"kind\":\"MyAPIObject\", \"apiVersion\":\"v1\", \"myPlugin\": { \"kind\":\"PluginA\", \"aOption\":\"foo\", }, }  So what happens? Decode first uses json or yaml to unmarshal the serialized data into your external MyAPIObject. That causes the raw JSON to be stored, but not unpacked. The next step is to copy (using pkg/conversion) into the internal struct. The runtime package's DefaultScheme has conversion functions installed which will unpack the JSON stored in RawExtension, turning it into the correct object type, and storing it in the Object. (TODO: In the case where the object is of an unknown type, a runtime.Unknown object will be created and stored.)  +k8s:deepcopy-gen=true +protobuf=true +k8s:openapi-gen=true | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


