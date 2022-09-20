/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GkeAutoprovisioningNodePoolDefaults : GKEAutoprovisioningNodePoolDefaults contains defaults for a node pool created by NAP.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GkeAutoprovisioningNodePoolDefaults {
    /// BootDiskKmsKey: The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cr yptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption
    #[serde(rename = "bootDiskKmsKey", skip_serializing_if = "Option::is_none")]
    pub boot_disk_kms_key: Option<String>,
    /// DiskSizeGb: Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB.
    #[serde(rename = "diskSizeGb", skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i64>,
    /// DiskType: Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard'
    #[serde(rename = "diskType", skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[serde(rename = "management", skip_serializing_if = "Option::is_none")]
    pub management: Option<Box<crate::models::GkeNodeManagement>>,
    /// MinCpuPlatform: Minimum CPU platform to be used for NAP created node pools. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as minCpuPlatform: Intel Haswell or minCpuPlatform: Intel Sandy Bridge. For more information, read how to specify min CPU platform (https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform) To unset the min cpu platform field pass \"automatic\" as field value.
    #[serde(rename = "minCpuPlatform", skip_serializing_if = "Option::is_none")]
    pub min_cpu_platform: Option<String>,
    /// OauthScopes: Scopes that are used by NAP when creating node pools.
    #[serde(rename = "oauthScopes", skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    /// ServiceAccount: The Google Cloud Platform Service Account to be used by the node VMs.
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    #[serde(rename = "shieldedInstanceConfig", skip_serializing_if = "Option::is_none")]
    pub shielded_instance_config: Option<Box<crate::models::GkeShieldedInstanceConfig>>,
    #[serde(rename = "upgradeSettings", skip_serializing_if = "Option::is_none")]
    pub upgrade_settings: Option<Box<crate::models::GkeUpgradeSettings>>,
}

impl GkeAutoprovisioningNodePoolDefaults {
    /// GKEAutoprovisioningNodePoolDefaults contains defaults for a node pool created by NAP.
    pub fn new() -> GkeAutoprovisioningNodePoolDefaults {
        GkeAutoprovisioningNodePoolDefaults {
            boot_disk_kms_key: None,
            disk_size_gb: None,
            disk_type: None,
            management: None,
            min_cpu_platform: None,
            oauth_scopes: None,
            service_account: None,
            shielded_instance_config: None,
            upgrade_settings: None,
        }
    }
}


