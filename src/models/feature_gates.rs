/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FeatureGates : FeatureGates represents an object holding feature gate settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeatureGates {
    #[serde(rename = "konnectivityService", skip_serializing_if = "Option::is_none")]
    pub konnectivity_service: Option<bool>,
    #[serde(rename = "oidcKubeCfgEndpoint", skip_serializing_if = "Option::is_none")]
    pub oidc_kube_cfg_endpoint: Option<bool>,
    #[serde(rename = "operatingSystemManager", skip_serializing_if = "Option::is_none")]
    pub operating_system_manager: Option<bool>,
}

impl FeatureGates {
    /// FeatureGates represents an object holding feature gate settings
    pub fn new() -> FeatureGates {
        FeatureGates {
            konnectivity_service: None,
            oidc_kube_cfg_endpoint: None,
            operating_system_manager: None,
        }
    }
}

