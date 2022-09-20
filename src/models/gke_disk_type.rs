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
pub struct GkeDiskType {
    /// DefaultDiskSizeGb: Server-defined default disk size in GB.
    #[serde(rename = "defaultDiskSizeGb", skip_serializing_if = "Option::is_none")]
    pub default_disk_size_gb: Option<i64>,
    /// Description: An optional description of this resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Kind: Type of the resource. Always compute#diskType for disk types.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GkeDiskType {
    pub fn new() -> GkeDiskType {
        GkeDiskType {
            default_disk_size_gb: None,
            description: None,
            kind: None,
            name: None,
        }
    }
}


