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
pub struct PublicCloudSpec {
    #[serde(rename = "alibaba", skip_serializing_if = "Option::is_none")]
    pub alibaba: Option<serde_json::Value>,
    #[serde(rename = "anexia", skip_serializing_if = "Option::is_none")]
    pub anexia: Option<serde_json::Value>,
    #[serde(rename = "aws", skip_serializing_if = "Option::is_none")]
    pub aws: Option<serde_json::Value>,
    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<crate::models::PublicAzureCloudSpec>>,
    #[serde(rename = "bringyourown", skip_serializing_if = "Option::is_none")]
    pub bringyourown: Option<serde_json::Value>,
    #[serde(rename = "dc", skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(rename = "digitalocean", skip_serializing_if = "Option::is_none")]
    pub digitalocean: Option<serde_json::Value>,
    #[serde(rename = "fake", skip_serializing_if = "Option::is_none")]
    pub fake: Option<serde_json::Value>,
    #[serde(rename = "gcp", skip_serializing_if = "Option::is_none")]
    pub gcp: Option<serde_json::Value>,
    #[serde(rename = "hetzner", skip_serializing_if = "Option::is_none")]
    pub hetzner: Option<serde_json::Value>,
    #[serde(rename = "kubevirt", skip_serializing_if = "Option::is_none")]
    pub kubevirt: Option<Box<crate::models::PublicKubevirtCloudSpec>>,
    #[serde(rename = "nutanix", skip_serializing_if = "Option::is_none")]
    pub nutanix: Option<serde_json::Value>,
    #[serde(rename = "openstack", skip_serializing_if = "Option::is_none")]
    pub openstack: Option<Box<crate::models::PublicOpenstackCloudSpec>>,
    #[serde(rename = "packet", skip_serializing_if = "Option::is_none")]
    pub packet: Option<serde_json::Value>,
    #[serde(rename = "vmwareclouddirector", skip_serializing_if = "Option::is_none")]
    pub vmwareclouddirector: Option<serde_json::Value>,
    #[serde(rename = "vsphere", skip_serializing_if = "Option::is_none")]
    pub vsphere: Option<serde_json::Value>,
}

impl PublicCloudSpec {
    pub fn new() -> PublicCloudSpec {
        PublicCloudSpec {
            alibaba: None,
            anexia: None,
            aws: None,
            azure: None,
            bringyourown: None,
            dc: None,
            digitalocean: None,
            fake: None,
            gcp: None,
            hetzner: None,
            kubevirt: None,
            nutanix: None,
            openstack: None,
            packet: None,
            vmwareclouddirector: None,
            vsphere: None,
        }
    }
}


