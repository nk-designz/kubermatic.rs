/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AddonSpec : AddonSpec addon specification



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddonSpec {
    /// ContinuouslyReconcile indicates that the addon cannot be deleted or modified outside of the UI after installation
    #[serde(rename = "continuouslyReconcile", skip_serializing_if = "Option::is_none")]
    pub continuously_reconcile: Option<bool>,
    /// IsDefault indicates whether the addon is default
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Variables is free form data to use for parsing the manifest templates
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl AddonSpec {
    /// AddonSpec addon specification
    pub fn new() -> AddonSpec {
        AddonSpec {
            continuously_reconcile: None,
            is_default: None,
            variables: None,
        }
    }
}

