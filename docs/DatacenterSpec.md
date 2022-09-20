# DatacenterSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alibaba** | Option<[**crate::models::DatacenterSpecAlibaba**](DatacenterSpecAlibaba.md)> |  | [optional]
**anexia** | Option<[**crate::models::DatacenterSpecAnexia**](DatacenterSpecAnexia.md)> |  | [optional]
**aws** | Option<[**crate::models::DatacenterSpecAws**](DatacenterSpecAWS.md)> |  | [optional]
**azure** | Option<[**crate::models::DatacenterSpecAzure**](DatacenterSpecAzure.md)> |  | [optional]
**bringyourown** | Option<[**serde_json::Value**](.md)> |  | [optional]
**country** | Option<**String**> | Optional: Country of the seed as ISO-3166 two-letter code, e.g. DE or UK. It is used for informational purposes. | [optional]
**digitalocean** | Option<[**crate::models::DatacenterSpecDigitalocean**](DatacenterSpecDigitalocean.md)> |  | [optional]
**enforce_audit_logging** | Option<**bool**> | EnforceAuditLogging enforces audit logging on every cluster within the DC, ignoring cluster-specific settings. | [optional]
**enforce_pod_security_policy** | Option<**bool**> | EnforcePodSecurityPolicy enforces pod security policy plugin on every clusters within the DC, ignoring cluster-specific settings | [optional]
**fake** | Option<[**crate::models::DatacenterSpecFake**](DatacenterSpecFake.md)> |  | [optional]
**gcp** | Option<[**crate::models::DatacenterSpecGcp**](DatacenterSpecGCP.md)> |  | [optional]
**hetzner** | Option<[**crate::models::DatacenterSpecHetzner**](DatacenterSpecHetzner.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | IPv6Enabled is a flag to indicate if the ipv6 is enabled for the datacenter. | [optional]
**kubevirt** | Option<[**crate::models::DatacenterSpecKubevirt**](DatacenterSpecKubevirt.md)> |  | [optional]
**location** | Option<**String**> | Optional: Detailed location of the cluster, like \"Hamburg\" or \"Datacenter 7\". It is used for informational purposes. | [optional]
**node** | Option<[**crate::models::NodeSettings**](NodeSettings.md)> |  | [optional]
**nutanix** | Option<[**crate::models::DatacenterSpecNutanix**](DatacenterSpecNutanix.md)> |  | [optional]
**openstack** | Option<[**crate::models::DatacenterSpecOpenstack**](DatacenterSpecOpenstack.md)> |  | [optional]
**packet** | Option<[**crate::models::DatacenterSpecPacket**](DatacenterSpecPacket.md)> |  | [optional]
**provider** | Option<**String**> | Name of the datacenter provider. Extracted based on which provider is defined in the spec. It is used for informational purposes. | [optional]
**required_emails** | Option<**Vec<String>**> |  | [optional]
**seed** | Option<**String**> | Name of the seed this datacenter belongs to. | [optional]
**vmwareclouddirector** | Option<[**crate::models::DatacenterSpecVMwareCloudDirector**](DatacenterSpecVMwareCloudDirector.md)> |  | [optional]
**vsphere** | Option<[**crate::models::DatacenterSpecVSphere**](DatacenterSpecVSphere.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


