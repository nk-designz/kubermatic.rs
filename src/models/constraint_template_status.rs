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
pub struct ConstraintTemplateStatus {
    #[serde(rename = "byPod", skip_serializing_if = "Option::is_none")]
    pub by_pod: Option<Vec<crate::models::ByPodStatus>>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
}

impl ConstraintTemplateStatus {
    pub fn new() -> ConstraintTemplateStatus {
        ConstraintTemplateStatus {
            by_pod: None,
            created: None,
        }
    }
}


