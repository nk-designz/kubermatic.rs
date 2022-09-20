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
pub struct ConstraintTemplateSelector {
    #[serde(rename = "labelSelector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<crate::models::LabelSelector>>,
    /// Providers is a list of cloud providers to which the Constraint Template applies to. Empty means all providers are selected.
    #[serde(rename = "providers", skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<String>>,
}

impl ConstraintTemplateSelector {
    pub fn new() -> ConstraintTemplateSelector {
        ConstraintTemplateSelector {
            label_selector: None,
            providers: None,
        }
    }
}


