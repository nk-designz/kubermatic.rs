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
pub struct Gke {
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
}

impl Gke {
    pub fn new() -> Gke {
        Gke {
            datacenter: None,
            enabled: None,
            service_account: None,
        }
    }
}

