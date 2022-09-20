/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SeedSettings : SeedSettings represents settings for a Seed cluster



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SeedSettings {
    #[serde(rename = "metering", skip_serializing_if = "Option::is_none")]
    pub metering: Option<Box<crate::models::MeteringConfiguration>>,
    #[serde(rename = "mla", skip_serializing_if = "Option::is_none")]
    pub mla: Option<Box<crate::models::Mla>>,
    /// the Seed level seed dns overwrite
    #[serde(rename = "seedDNSOverwrite", skip_serializing_if = "Option::is_none")]
    pub seed_dns_overwrite: Option<String>,
}

impl SeedSettings {
    /// SeedSettings represents settings for a Seed cluster
    pub fn new() -> SeedSettings {
        SeedSettings {
            metering: None,
            mla: None,
            seed_dns_overwrite: None,
        }
    }
}

