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
pub struct ClusterTemplateNodeDeployment {
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<crate::models::NodeDeploymentSpec>>,
}

impl ClusterTemplateNodeDeployment {
    pub fn new() -> ClusterTemplateNodeDeployment {
        ClusterTemplateNodeDeployment {
            spec: None,
        }
    }
}


