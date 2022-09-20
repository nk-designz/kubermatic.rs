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
pub struct DatacenterSpecDigitalocean {
    /// Datacenter location, e.g. \"ams3\". A list of existing datacenters can be found at https://www.digitalocean.com/docs/platform/availability-matrix/
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl DatacenterSpecDigitalocean {
    pub fn new() -> DatacenterSpecDigitalocean {
        DatacenterSpecDigitalocean {
            region: None,
        }
    }
}


