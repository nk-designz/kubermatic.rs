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
pub struct DatacenterSpecAzure {
    /// Region to use, for example \"westeurope\". A list of available regions can be found at https://azure.microsoft.com/en-us/global-infrastructure/locations/
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl DatacenterSpecAzure {
    pub fn new() -> DatacenterSpecAzure {
        DatacenterSpecAzure {
            location: None,
        }
    }
}


