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
pub struct GcpMachineSize {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "vcpus", skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
}

impl GcpMachineSize {
    pub fn new() -> GcpMachineSize {
        GcpMachineSize {
            description: None,
            memory: None,
            name: None,
            vcpus: None,
        }
    }
}


