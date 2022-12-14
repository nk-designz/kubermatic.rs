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
pub struct Hetzner {
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Network is the pre-existing Hetzner network in which the machines are running. While machines can be in multiple networks, a single one must be chosen for the HCloud CCM to work. If this is empty, the network configured on the datacenter will be used.
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Token is used to authenticate with the Hetzner API.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl Hetzner {
    pub fn new() -> Hetzner {
        Hetzner {
            datacenter: None,
            enabled: None,
            network: None,
            token: None,
        }
    }
}


