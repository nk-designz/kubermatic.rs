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
pub struct ClusterTemplateInfo {
    /// indicates the preset name
    #[serde(rename = "credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    #[serde(rename = "inheritedLabels", skip_serializing_if = "Option::is_none")]
    pub inherited_labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<crate::models::ClusterSpec>>,
}

impl ClusterTemplateInfo {
    pub fn new() -> ClusterTemplateInfo {
        ClusterTemplateInfo {
            credential: None,
            inherited_labels: None,
            labels: None,
            spec: None,
        }
    }
}


