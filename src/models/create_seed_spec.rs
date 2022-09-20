/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSeedSpec {
    /// Optional: Country of the seed as ISO-3166 two-letter code, e.g. DE or UK. For informational purposes in the Kubermatic dashboard only.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// DefaultClusterTemplate is the name of a cluster template of scope \"seed\" that is used to default all new created clusters
    #[serde(rename = "defaultClusterTemplate", skip_serializing_if = "Option::is_none")]
    pub default_cluster_template: Option<String>,
    /// Possible values are `NodePort`, `LoadBalancer` or `Tunneling` (requires a feature gate).
    #[serde(rename = "expose_strategy", skip_serializing_if = "Option::is_none")]
    pub expose_strategy: Option<String>,
    /// The raw Kubeconfig encoded to base64. This field is used for cluster creation or update.
    #[serde(rename = "kubeconfig", skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<String>,
    /// Optional: Detailed location of the cluster, like \"Hamburg\" or \"Datacenter 7\". For informational purposes in the Kubermatic dashboard only.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "mla", skip_serializing_if = "Option::is_none")]
    pub mla: Option<Box<crate::models::CreateSeedMlaSettings>>,
    #[serde(rename = "proxy_settings", skip_serializing_if = "Option::is_none")]
    pub proxy_settings: Option<Box<crate::models::CreateSeedProxySettings>>,
    /// Optional: This can be used to override the DNS name used for this seed. By default the seed name is used.
    #[serde(rename = "seed_dns_overwrite", skip_serializing_if = "Option::is_none")]
    pub seed_dns_overwrite: Option<String>,
}

impl CreateSeedSpec {
    pub fn new() -> CreateSeedSpec {
        CreateSeedSpec {
            country: None,
            default_cluster_template: None,
            expose_strategy: None,
            kubeconfig: None,
            location: None,
            mla: None,
            proxy_settings: None,
            seed_dns_overwrite: None,
        }
    }
}

