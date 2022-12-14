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
pub struct ContainerdRegistry {
    /// List of registry mirrors to use
    #[serde(rename = "mirrors", skip_serializing_if = "Option::is_none")]
    pub mirrors: Option<Vec<String>>,
}

impl ContainerdRegistry {
    pub fn new() -> ContainerdRegistry {
        ContainerdRegistry {
            mirrors: None,
        }
    }
}


