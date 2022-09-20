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
pub struct KubeOneAwsCloudSpec {
    #[serde(rename = "accessKeyID", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "secretAccessKey", skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

impl KubeOneAwsCloudSpec {
    pub fn new() -> KubeOneAwsCloudSpec {
        KubeOneAwsCloudSpec {
            access_key_id: None,
            secret_access_key: None,
        }
    }
}


