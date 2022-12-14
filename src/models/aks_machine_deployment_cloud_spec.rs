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
pub struct AksMachineDeploymentCloudSpec {
    #[serde(rename = "basicSettings", skip_serializing_if = "Option::is_none")]
    pub basic_settings: Option<Box<crate::models::AgentPoolBasics>>,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<crate::models::AgentPoolConfig>>,
    /// Name - Node pool name must contain only lowercase letters and numbers. For Linux node pools must be 12 or fewer characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optionalSettings", skip_serializing_if = "Option::is_none")]
    pub optional_settings: Option<Box<crate::models::AgentPoolOptionalSettings>>,
}

impl AksMachineDeploymentCloudSpec {
    pub fn new() -> AksMachineDeploymentCloudSpec {
        AksMachineDeploymentCloudSpec {
            basic_settings: None,
            configuration: None,
            name: None,
            optional_settings: None,
        }
    }
}


