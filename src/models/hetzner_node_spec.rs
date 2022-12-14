/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HetznerNodeSpec : HetznerNodeSpec Hetzner node settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HetznerNodeSpec {
    /// network name
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// server type
    #[serde(rename = "type")]
    pub _type: String,
}

impl HetznerNodeSpec {
    /// HetznerNodeSpec Hetzner node settings
    pub fn new(_type: String) -> HetznerNodeSpec {
        HetznerNodeSpec {
            network: None,
            _type,
        }
    }
}


