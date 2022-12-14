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
pub struct PublicAzureCloudSpec {
    #[serde(rename = "assignAvailabilitySet", skip_serializing_if = "Option::is_none")]
    pub assign_availability_set: Option<bool>,
}

impl PublicAzureCloudSpec {
    pub fn new() -> PublicAzureCloudSpec {
        PublicAzureCloudSpec {
            assign_availability_set: None,
        }
    }
}


