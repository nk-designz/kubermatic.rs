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
pub struct NetworkRanges {
    #[serde(rename = "cidrBlocks", skip_serializing_if = "Option::is_none")]
    pub cidr_blocks: Option<Vec<String>>,
}

impl NetworkRanges {
    pub fn new() -> NetworkRanges {
        NetworkRanges {
            cidr_blocks: None,
        }
    }
}


