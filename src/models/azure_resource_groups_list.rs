/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AzureResourceGroupsList : AzureResourceGroupsList is the object representing the resource groups for vms in azure cloud provider



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AzureResourceGroupsList {
    #[serde(rename = "resourceGroups", skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<Vec<String>>,
}

impl AzureResourceGroupsList {
    /// AzureResourceGroupsList is the object representing the resource groups for vms in azure cloud provider
    pub fn new() -> AzureResourceGroupsList {
        AzureResourceGroupsList {
            resource_groups: None,
        }
    }
}


