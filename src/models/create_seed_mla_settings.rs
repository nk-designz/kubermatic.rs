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
pub struct CreateSeedMlaSettings {
    /// Optional: UserClusterMLAEnabled controls whether the user cluster MLA (Monitoring, Logging & Alerting) stack is enabled in the seed.
    #[serde(rename = "userClusterMLAEnabled", skip_serializing_if = "Option::is_none")]
    pub user_cluster_mla_enabled: Option<bool>,
}

impl CreateSeedMlaSettings {
    pub fn new() -> CreateSeedMlaSettings {
        CreateSeedMlaSettings {
            user_cluster_mla_enabled: None,
        }
    }
}


