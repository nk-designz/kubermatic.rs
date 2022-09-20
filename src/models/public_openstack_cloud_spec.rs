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
pub struct PublicOpenstackCloudSpec {
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "floatingIPPool", skip_serializing_if = "Option::is_none")]
    pub floating_ip_pool: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "projectID", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "routerID", skip_serializing_if = "Option::is_none")]
    pub router_id: Option<String>,
    #[serde(rename = "securityGroups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<String>,
    #[serde(rename = "subnetID", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

impl PublicOpenstackCloudSpec {
    pub fn new() -> PublicOpenstackCloudSpec {
        PublicOpenstackCloudSpec {
            domain: None,
            floating_ip_pool: None,
            network: None,
            project: None,
            project_id: None,
            router_id: None,
            security_groups: None,
            subnet_id: None,
        }
    }
}

