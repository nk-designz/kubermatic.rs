# SeedSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country** | Option<**String**> | Optional: Country of the seed as ISO-3166 two-letter code, e.g. DE or UK. For informational purposes in the Kubermatic dashboard only. | [optional]
**datacenters** | Option<[**::std::collections::HashMap<String, crate::models::Datacenter>**](Datacenter.md)> | Datacenters contains a map of the possible datacenters (DCs) in this seed. Each DC must have a globally unique identifier (i.e. names must be unique across all seeds). | [optional]
**etcd_backup_restore** | Option<[**crate::models::EtcdBackupRestore**](EtcdBackupRestore.md)> |  | [optional]
**expose_strategy** | Option<**String**> | Possible values are `NodePort`, `LoadBalancer` or `Tunneling` (requires a feature gate). | [optional]
**kubeconfig** | Option<[**crate::models::ObjectReference**](ObjectReference.md)> |  | [optional]
**location** | Option<**String**> | Optional: Detailed location of the cluster, like \"Hamburg\" or \"Datacenter 7\". For informational purposes in the Kubermatic dashboard only. | [optional]
**mla** | Option<[**crate::models::SeedMlaSettings**](SeedMLASettings.md)> |  | [optional]
**proxy_settings** | Option<[**crate::models::ProxySettings**](ProxySettings.md)> |  | [optional]
**seed_dns_overwrite** | Option<**String**> | Optional: This can be used to override the DNS name used for this seed. By default the seed name is used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


