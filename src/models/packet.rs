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
pub struct Packet {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "billingCycle", skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "projectID", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            api_key: None,
            billing_cycle: None,
            datacenter: None,
            enabled: None,
            project_id: None,
        }
    }
}

