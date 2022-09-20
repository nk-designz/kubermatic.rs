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
pub struct KubeOneEquinixCloudSpec {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "projectID", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl KubeOneEquinixCloudSpec {
    pub fn new() -> KubeOneEquinixCloudSpec {
        KubeOneEquinixCloudSpec {
            api_key: None,
            project_id: None,
        }
    }
}


