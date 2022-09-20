/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AzureSubnetsList : AzureSubnetsList is the object representing the subnets for vms in azure cloud provider



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AzureSubnetsList {
    #[serde(rename = "subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

impl AzureSubnetsList {
    /// AzureSubnetsList is the object representing the subnets for vms in azure cloud provider
    pub fn new() -> AzureSubnetsList {
        AzureSubnetsList {
            subnets: None,
        }
    }
}

