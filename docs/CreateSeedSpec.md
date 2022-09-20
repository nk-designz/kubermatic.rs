# CreateSeedSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country** | Option<**String**> | Optional: Country of the seed as ISO-3166 two-letter code, e.g. DE or UK. For informational purposes in the Kubermatic dashboard only. | [optional]
**default_cluster_template** | Option<**String**> | DefaultClusterTemplate is the name of a cluster template of scope \"seed\" that is used to default all new created clusters | [optional]
**expose_strategy** | Option<**String**> | Possible values are `NodePort`, `LoadBalancer` or `Tunneling` (requires a feature gate). | [optional]
**kubeconfig** | Option<**String**> | The raw Kubeconfig encoded to base64. This field is used for cluster creation or update. | [optional]
**location** | Option<**String**> | Optional: Detailed location of the cluster, like \"Hamburg\" or \"Datacenter 7\". For informational purposes in the Kubermatic dashboard only. | [optional]
**mla** | Option<[**crate::models::CreateSeedMlaSettings**](CreateSeedMLASettings.md)> |  | [optional]
**proxy_settings** | Option<[**crate::models::CreateSeedProxySettings**](CreateSeedProxySettings.md)> |  | [optional]
**seed_dns_overwrite** | Option<**String**> | Optional: This can be used to override the DNS name used for this seed. By default the seed name is used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


