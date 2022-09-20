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
pub struct DatacenterSpecAnexia {
    /// LocationID the location of the region
    #[serde(rename = "locationID", skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
}

impl DatacenterSpecAnexia {
    pub fn new() -> DatacenterSpecAnexia {
        DatacenterSpecAnexia {
            location_id: None,
        }
    }
}


