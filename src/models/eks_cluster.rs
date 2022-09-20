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
pub struct EksCluster {
    #[serde(rename = "imported", skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl EksCluster {
    pub fn new() -> EksCluster {
        EksCluster {
            imported: None,
            name: None,
            region: None,
        }
    }
}

