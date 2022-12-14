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
pub struct KubeOneSpec {
    #[serde(rename = "cloudSpec", skip_serializing_if = "Option::is_none")]
    pub cloud_spec: Option<Box<crate::models::KubeOneCloudSpec>>,
    #[serde(rename = "containerRuntime", skip_serializing_if = "Option::is_none")]
    pub container_runtime: Option<String>,
    /// Manifest Base64 encoded manifest
    #[serde(rename = "manifest", skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    #[serde(rename = "sshKey", skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<Box<crate::models::KubeOneSshKey>>,
}

impl KubeOneSpec {
    pub fn new() -> KubeOneSpec {
        KubeOneSpec {
            cloud_spec: None,
            container_runtime: None,
            manifest: None,
            ssh_key: None,
        }
    }
}


