/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationSpec : ApplicationSpec represents the specification for an application



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationSpec {
    #[serde(rename = "applicationRef", skip_serializing_if = "Option::is_none")]
    pub application_ref: Option<Box<crate::models::ApplicationRef>>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Box<crate::models::NamespaceSpec>>,
    /// Values describe overrides for manifest-rendering
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
}

impl ApplicationSpec {
    /// ApplicationSpec represents the specification for an application
    pub fn new() -> ApplicationSpec {
        ApplicationSpec {
            application_ref: None,
            namespace: None,
            values: None,
        }
    }
}


