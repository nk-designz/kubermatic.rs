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
pub struct PacketCloudSpec {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "billingCycle", skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(rename = "credentialsReference", skip_serializing_if = "Option::is_none")]
    pub credentials_reference: Option<Box<crate::models::GlobalObjectKeySelector>>,
    #[serde(rename = "projectID", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl PacketCloudSpec {
    pub fn new() -> PacketCloudSpec {
        PacketCloudSpec {
            api_key: None,
            billing_cycle: None,
            credentials_reference: None,
            project_id: None,
        }
    }
}


