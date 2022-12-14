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
pub struct AksLocation {
    /// The location name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// READ-ONLY; The category of the region.
    #[serde(rename = "regionCategory", skip_serializing_if = "Option::is_none")]
    pub region_category: Option<String>,
}

impl AksLocation {
    pub fn new() -> AksLocation {
        AksLocation {
            name: None,
            region_category: None,
        }
    }
}


