/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ByPodStatus : ByPodStatus defines the observed state of ConstraintTemplate as seen by an individual controller +kubebuilder:pruning:PreserveUnknownFields



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ByPodStatus {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::CreateCrdError>>,
    /// a unique identifier for the pod that wrote the status
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "observedGeneration", skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}

impl ByPodStatus {
    /// ByPodStatus defines the observed state of ConstraintTemplate as seen by an individual controller +kubebuilder:pruning:PreserveUnknownFields
    pub fn new() -> ByPodStatus {
        ByPodStatus {
            errors: None,
            id: None,
            observed_generation: None,
        }
    }
}


