/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MachineDeploymentStatus : [MachineDeploymentStatus] MachineDeploymentStatus defines the observed state of MachineDeployment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MachineDeploymentStatus {
    /// Total number of available machines (ready for at least minReadySeconds) targeted by this deployment. +optional
    #[serde(rename = "availableReplicas", skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    /// The generation observed by the deployment controller. +optional
    #[serde(rename = "observedGeneration", skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// Total number of ready machines targeted by this deployment. +optional
    #[serde(rename = "readyReplicas", skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// Total number of non-terminated machines targeted by this deployment (their labels match the selector). +optional
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Total number of unavailable machines targeted by this deployment. This is the total number of machines that are still required for the deployment to have 100% available capacity. They may either be machines that are running but not yet available or machines that still have not been created. +optional
    #[serde(rename = "unavailableReplicas", skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,
    /// Total number of non-terminated machines targeted by this deployment that have the desired template spec. +optional
    #[serde(rename = "updatedReplicas", skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
}

impl MachineDeploymentStatus {
    /// [MachineDeploymentStatus] MachineDeploymentStatus defines the observed state of MachineDeployment.
    pub fn new() -> MachineDeploymentStatus {
        MachineDeploymentStatus {
            available_replicas: None,
            observed_generation: None,
            ready_replicas: None,
            replicas: None,
            unavailable_replicas: None,
            updated_replicas: None,
        }
    }
}


