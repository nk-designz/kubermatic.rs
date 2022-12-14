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
pub struct OperatingSystemSpec {
    #[serde(rename = "amzn2", skip_serializing_if = "Option::is_none")]
    pub amzn2: Option<Box<crate::models::AmazonLinuxSpec>>,
    #[serde(rename = "centos", skip_serializing_if = "Option::is_none")]
    pub centos: Option<Box<crate::models::CentOsSpec>>,
    #[serde(rename = "flatcar", skip_serializing_if = "Option::is_none")]
    pub flatcar: Option<Box<crate::models::FlatcarSpec>>,
    #[serde(rename = "rhel", skip_serializing_if = "Option::is_none")]
    pub rhel: Option<Box<crate::models::RhelSpec>>,
    #[serde(rename = "rockylinux", skip_serializing_if = "Option::is_none")]
    pub rockylinux: Option<Box<crate::models::RockyLinuxSpec>>,
    #[serde(rename = "sles", skip_serializing_if = "Option::is_none")]
    pub sles: Option<Box<crate::models::SlesSpec>>,
    #[serde(rename = "ubuntu", skip_serializing_if = "Option::is_none")]
    pub ubuntu: Option<Box<crate::models::UbuntuSpec>>,
}

impl OperatingSystemSpec {
    pub fn new() -> OperatingSystemSpec {
        OperatingSystemSpec {
            amzn2: None,
            centos: None,
            flatcar: None,
            rhel: None,
            rockylinux: None,
            sles: None,
            ubuntu: None,
        }
    }
}


