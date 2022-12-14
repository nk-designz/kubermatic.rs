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
pub struct ConstraintSpec {
    /// ConstraintType specifies the type of gatekeeper constraint that the constraint applies to
    #[serde(rename = "constraintType", skip_serializing_if = "Option::is_none")]
    pub constraint_type: Option<String>,
    /// Disabled  is the flag for disabling OPA constraints
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<Box<crate::models::ModelMatch>>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<crate::models::Parameters>,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<Box<crate::models::ConstraintSelector>>,
}

impl ConstraintSpec {
    pub fn new() -> ConstraintSpec {
        ConstraintSpec {
            constraint_type: None,
            disabled: None,
            _match: None,
            parameters: None,
            selector: None,
        }
    }
}


