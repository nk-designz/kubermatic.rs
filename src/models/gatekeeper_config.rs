/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GatekeeperConfig : GatekeeperConfig represents a gatekeeper config



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GatekeeperConfig {
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<crate::models::GatekeeperConfigSpec>>,
}

impl GatekeeperConfig {
    /// GatekeeperConfig represents a gatekeeper config
    pub fn new() -> GatekeeperConfig {
        GatekeeperConfig {
            spec: None,
        }
    }
}


