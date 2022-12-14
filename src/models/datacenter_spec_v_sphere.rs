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
pub struct DatacenterSpecVSphere {
    /// If set to true, disables the TLS certificate check against the endpoint.
    #[serde(rename = "allowInsecure", skip_serializing_if = "Option::is_none")]
    pub allow_insecure: Option<bool>,
    /// The name of the vSphere cluster to use. Used for out-of-tree CSI Driver.
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// The name of the datacenter to use.
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// The default Datastore to be used for provisioning volumes using storage classes/dynamic provisioning and for storing virtual machine files in case no `Datastore` or `DatastoreCluster` is provided at Cluster level.
    #[serde(rename = "datastore", skip_serializing_if = "Option::is_none")]
    pub datastore: Option<String>,
    /// Endpoint URL to use, including protocol, for example \"https://vcenter.example.com\".
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "infraManagementUser", skip_serializing_if = "Option::is_none")]
    pub infra_management_user: Option<Box<crate::models::VSphereCredentials>>,
    /// Optional: defines if the IPv6 is enabled for the datacenter
    #[serde(rename = "ipv6Enabled", skip_serializing_if = "Option::is_none")]
    pub ipv6_enabled: Option<bool>,
    /// Optional: The root path for cluster specific VM folders. Each cluster gets its own folder below the root folder. Must be the FQDN (for example \"/datacenter-1/vm/all-kubermatic-vms-in-here\") and defaults to the root VM folder: \"/datacenter-1/vm\"
    #[serde(rename = "rootPath", skip_serializing_if = "Option::is_none")]
    pub root_path: Option<String>,
    /// The name of the storage policy to use for the storage class created in the user cluster.
    #[serde(rename = "storagePolicy", skip_serializing_if = "Option::is_none")]
    pub storage_policy: Option<String>,
    #[serde(rename = "templates", skip_serializing_if = "Option::is_none")]
    pub templates: Option<crate::models::ImageList>,
}

impl DatacenterSpecVSphere {
    pub fn new() -> DatacenterSpecVSphere {
        DatacenterSpecVSphere {
            allow_insecure: None,
            cluster: None,
            datacenter: None,
            datastore: None,
            endpoint: None,
            infra_management_user: None,
            ipv6_enabled: None,
            root_path: None,
            storage_policy: None,
            templates: None,
        }
    }
}


