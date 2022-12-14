/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NutanixNodeSpec : NutanixNodeSpec nutanix specific node settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NutanixNodeSpec {
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "cpuCores", skip_serializing_if = "Option::is_none")]
    pub cpu_cores: Option<i64>,
    #[serde(rename = "cpuPassthrough", skip_serializing_if = "Option::is_none")]
    pub cpu_passthrough: Option<bool>,
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i64>,
    #[serde(rename = "diskSize", skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    #[serde(rename = "imageName", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(rename = "memoryMB", skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i64>,
    #[serde(rename = "subnetName", skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
}

impl NutanixNodeSpec {
    /// NutanixNodeSpec nutanix specific node settings
    pub fn new() -> NutanixNodeSpec {
        NutanixNodeSpec {
            categories: None,
            cpu_cores: None,
            cpu_passthrough: None,
            cpus: None,
            disk_size: None,
            image_name: None,
            memory_mb: None,
            subnet_name: None,
        }
    }
}


