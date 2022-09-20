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
pub struct ConstraintBody {
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<crate::models::ConstraintSpec>>,
    /// Name is the name for the constraint
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ConstraintBody {
    pub fn new() -> ConstraintBody {
        ConstraintBody {
            spec: None,
            name: None,
        }
    }
}


