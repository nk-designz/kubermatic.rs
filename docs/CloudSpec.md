# CloudSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alibaba** | Option<[**crate::models::AlibabaCloudSpec**](AlibabaCloudSpec.md)> |  | [optional]
**anexia** | Option<[**crate::models::AnexiaCloudSpec**](AnexiaCloudSpec.md)> |  | [optional]
**aws** | Option<[**crate::models::AwsCloudSpec**](AWSCloudSpec.md)> |  | [optional]
**azure** | Option<[**crate::models::AzureCloudSpec**](AzureCloudSpec.md)> |  | [optional]
**bringyourown** | Option<[**serde_json::Value**](.md)> |  | [optional]
**dc** | Option<**String**> | DatacenterName states the name of a cloud provider \"datacenter\" (defined in `Seed` resources) this cluster should be deployed into. | [optional]
**digitalocean** | Option<[**crate::models::DigitaloceanCloudSpec**](DigitaloceanCloudSpec.md)> |  | [optional]
**fake** | Option<[**crate::models::FakeCloudSpec**](FakeCloudSpec.md)> |  | [optional]
**gcp** | Option<[**crate::models::GcpCloudSpec**](GCPCloudSpec.md)> |  | [optional]
**hetzner** | Option<[**crate::models::HetznerCloudSpec**](HetznerCloudSpec.md)> |  | [optional]
**kubevirt** | Option<[**crate::models::KubevirtCloudSpec**](KubevirtCloudSpec.md)> |  | [optional]
**nutanix** | Option<[**crate::models::NutanixCloudSpec**](NutanixCloudSpec.md)> |  | [optional]
**openstack** | Option<[**crate::models::OpenstackCloudSpec**](OpenstackCloudSpec.md)> |  | [optional]
**packet** | Option<[**crate::models::PacketCloudSpec**](PacketCloudSpec.md)> |  | [optional]
**provider_name** | Option<**String**> | ProviderName is the name of the cloud provider used for this cluster. This must match the given provider spec (e.g. if the providerName is \"aws\", then the `aws` field must be set). | [optional]
**vmwareclouddirector** | Option<[**crate::models::VMwareCloudDirectorCloudSpec**](VMwareCloudDirectorCloudSpec.md)> |  | [optional]
**vsphere** | Option<[**crate::models::VSphereCloudSpec**](VSphereCloudSpec.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


