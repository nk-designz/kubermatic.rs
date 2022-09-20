/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AmazonLinuxSpec : AmazonLinuxSpec amazon linux specific settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AmazonLinuxSpec {
    /// do a dist-upgrade on boot and reboot it required afterwards
    #[serde(rename = "distUpgradeOnBoot", skip_serializing_if = "Option::is_none")]
    pub dist_upgrade_on_boot: Option<bool>,
}

impl AmazonLinuxSpec {
    /// AmazonLinuxSpec amazon linux specific settings
    pub fn new() -> AmazonLinuxSpec {
        AmazonLinuxSpec {
            dist_upgrade_on_boot: None,
        }
    }
}


