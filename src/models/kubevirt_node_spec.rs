/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KubevirtNodeSpec : KubevirtNodeSpec kubevirt specific node settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KubevirtNodeSpec {
    /// CPUs states how many cpus the kubevirt node will have.
    #[serde(rename = "cpus")]
    pub cpus: String,
    /// FlavorName states name of the virtual-machine flavor.
    #[serde(rename = "flavorName", skip_serializing_if = "Option::is_none")]
    pub flavor_name: Option<String>,
    /// FlavorProfile states name of virtual-machine profile.
    #[serde(rename = "flavorProfile", skip_serializing_if = "Option::is_none")]
    pub flavor_profile: Option<String>,
    /// Memory states the memory that kubevirt node will have.
    #[serde(rename = "memory")]
    pub memory: String,
    #[serde(rename = "nodeAffinityPreset", skip_serializing_if = "Option::is_none")]
    pub node_affinity_preset: Option<Box<crate::models::NodeAffinityPreset>>,
    /// PodAffinityPreset describes pod affinity scheduling rules
    #[serde(rename = "podAffinityPreset", skip_serializing_if = "Option::is_none")]
    pub pod_affinity_preset: Option<String>,
    /// PodAntiAffinityPreset describes pod anti-affinity scheduling rules
    #[serde(rename = "podAntiAffinityPreset", skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity_preset: Option<String>,
    /// PrimaryDiskOSImage states the source from which the imported image will be downloaded. This field contains: a URL to download an Os Image from a HTTP source. a DataVolume Name as source for DataVolume cloning.
    #[serde(rename = "primaryDiskOSImage")]
    pub primary_disk_os_image: String,
    /// PrimaryDiskSize states the size of the provisioned pvc per node.
    #[serde(rename = "primaryDiskSize")]
    pub primary_disk_size: String,
    /// PrimaryDiskStorageClassName states the storage class name for the provisioned PVCs.
    #[serde(rename = "primaryDiskStorageClassName")]
    pub primary_disk_storage_class_name: String,
    /// SecondaryDisks contains list of secondary-disks
    #[serde(rename = "secondaryDisks", skip_serializing_if = "Option::is_none")]
    pub secondary_disks: Option<Vec<crate::models::SecondaryDisks>>,
}

impl KubevirtNodeSpec {
    /// KubevirtNodeSpec kubevirt specific node settings
    pub fn new(cpus: String, memory: String, primary_disk_os_image: String, primary_disk_size: String, primary_disk_storage_class_name: String) -> KubevirtNodeSpec {
        KubevirtNodeSpec {
            cpus,
            flavor_name: None,
            flavor_profile: None,
            memory,
            node_affinity_preset: None,
            pod_affinity_preset: None,
            pod_anti_affinity_preset: None,
            primary_disk_os_image,
            primary_disk_size,
            primary_disk_storage_class_name,
            secondary_disks: None,
        }
    }
}


