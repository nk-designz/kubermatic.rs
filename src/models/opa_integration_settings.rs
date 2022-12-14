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
pub struct OpaIntegrationSettings {
    #[serde(rename = "auditResources", skip_serializing_if = "Option::is_none")]
    pub audit_resources: Option<Box<crate::models::ResourceRequirements>>,
    #[serde(rename = "controllerResources", skip_serializing_if = "Option::is_none")]
    pub controller_resources: Option<Box<crate::models::ResourceRequirements>>,
    /// Enables OPA Gatekeeper integration.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Optional: Enables experimental mutation in Gatekeeper.
    #[serde(rename = "experimentalEnableMutation", skip_serializing_if = "Option::is_none")]
    pub experimental_enable_mutation: Option<bool>,
    /// The timeout in seconds that is set for the Gatekeeper validating webhook admission review calls. Defaults to `10` (seconds).
    #[serde(rename = "webhookTimeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub webhook_timeout_seconds: Option<i32>,
}

impl OpaIntegrationSettings {
    pub fn new() -> OpaIntegrationSettings {
        OpaIntegrationSettings {
            audit_resources: None,
            controller_resources: None,
            enabled: None,
            experimental_enable_mutation: None,
            webhook_timeout_seconds: None,
        }
    }
}


