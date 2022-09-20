/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NodeDeploymentSpec : NodeDeploymentSpec node deployment specification



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NodeDeploymentSpec {
    /// Only supported for nodes with Kubernetes 1.23 or less.
    #[serde(rename = "dynamicConfig", skip_serializing_if = "Option::is_none")]
    pub dynamic_config: Option<bool>,
    #[serde(rename = "paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(rename = "replicas")]
    pub replicas: i32,
    #[serde(rename = "template")]
    pub template: Box<crate::models::NodeSpec>,
}

impl NodeDeploymentSpec {
    /// NodeDeploymentSpec node deployment specification
    pub fn new(replicas: i32, template: crate::models::NodeSpec) -> NodeDeploymentSpec {
        NodeDeploymentSpec {
            dynamic_config: None,
            paused: None,
            replicas,
            template: Box::new(template),
        }
    }
}


