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
pub struct OpenstackNetwork {
    /// External set if network is the external network
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    /// Id uniquely identifies the current network
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name is the name of the network
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl OpenstackNetwork {
    pub fn new() -> OpenstackNetwork {
        OpenstackNetwork {
            external: None,
            id: None,
            name: None,
        }
    }
}


