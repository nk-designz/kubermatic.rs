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
pub struct KubeOneCloudSpec {
    #[serde(rename = "aws", skip_serializing_if = "Option::is_none")]
    pub aws: Option<Box<crate::models::KubeOneAwsCloudSpec>>,
    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<crate::models::KubeOneAzureCloudSpec>>,
    #[serde(rename = "digitalocean", skip_serializing_if = "Option::is_none")]
    pub digitalocean: Option<Box<crate::models::KubeOneDigitalOceanCloudSpec>>,
    #[serde(rename = "equinix", skip_serializing_if = "Option::is_none")]
    pub equinix: Option<Box<crate::models::KubeOneEquinixCloudSpec>>,
    #[serde(rename = "gcp", skip_serializing_if = "Option::is_none")]
    pub gcp: Option<Box<crate::models::KubeOneGcpCloudSpec>>,
    #[serde(rename = "hetzner", skip_serializing_if = "Option::is_none")]
    pub hetzner: Option<Box<crate::models::KubeOneHetznerCloudSpec>>,
    #[serde(rename = "nutanix", skip_serializing_if = "Option::is_none")]
    pub nutanix: Option<Box<crate::models::KubeOneNutanixCloudSpec>>,
    #[serde(rename = "openstack", skip_serializing_if = "Option::is_none")]
    pub openstack: Option<Box<crate::models::KubeOneOpenStackCloudSpec>>,
    #[serde(rename = "vmwareclouddirector", skip_serializing_if = "Option::is_none")]
    pub vmwareclouddirector: Option<Box<crate::models::KubeOneVMwareCloudDirectorCloudSpec>>,
    #[serde(rename = "vsphere", skip_serializing_if = "Option::is_none")]
    pub vsphere: Option<Box<crate::models::KubeOneVSphereCloudSpec>>,
}

impl KubeOneCloudSpec {
    pub fn new() -> KubeOneCloudSpec {
        KubeOneCloudSpec {
            aws: None,
            azure: None,
            digitalocean: None,
            equinix: None,
            gcp: None,
            hetzner: None,
            nutanix: None,
            openstack: None,
            vmwareclouddirector: None,
            vsphere: None,
        }
    }
}


