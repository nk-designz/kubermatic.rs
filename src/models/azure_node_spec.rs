/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AzureNodeSpec : AzureNodeSpec describes settings for an Azure node



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AzureNodeSpec {
    /// AssignAvailabilitySet is used to check if an availability set should be created and assigned to the cluster.
    #[serde(rename = "assignAvailabilitySet", skip_serializing_if = "Option::is_none")]
    pub assign_availability_set: Option<bool>,
    /// should the machine have a publicly accessible IP address
    #[serde(rename = "assignPublicIP", skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<bool>,
    /// Data disk size in GB
    #[serde(rename = "dataDiskSize", skip_serializing_if = "Option::is_none")]
    pub data_disk_size: Option<i32>,
    /// ImageID represents the ID of the image that should be used to run the node
    #[serde(rename = "imageID", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// OS disk size in GB
    #[serde(rename = "osDiskSize", skip_serializing_if = "Option::is_none")]
    pub os_disk_size: Option<i32>,
    /// VM size
    #[serde(rename = "size")]
    pub size: String,
    /// Additional metadata to set
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// Zones represents the availability zones for azure vms
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

impl AzureNodeSpec {
    /// AzureNodeSpec describes settings for an Azure node
    pub fn new(size: String) -> AzureNodeSpec {
        AzureNodeSpec {
            assign_availability_set: None,
            assign_public_ip: None,
            data_disk_size: None,
            image_id: None,
            os_disk_size: None,
            size,
            tags: None,
            zones: None,
        }
    }
}


