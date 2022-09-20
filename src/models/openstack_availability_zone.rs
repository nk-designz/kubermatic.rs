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
pub struct OpenstackAvailabilityZone {
    /// Name is the name of the availability zone
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl OpenstackAvailabilityZone {
    pub fn new() -> OpenstackAvailabilityZone {
        OpenstackAvailabilityZone {
            name: None,
        }
    }
}


