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
pub struct AwsCloudSpec {
    #[serde(rename = "accessKeyID", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "assumeRoleARN", skip_serializing_if = "Option::is_none")]
    pub assume_role_arn: Option<String>,
    #[serde(rename = "assumeRoleExternalID", skip_serializing_if = "Option::is_none")]
    pub assume_role_external_id: Option<String>,
    #[serde(rename = "credentialsReference", skip_serializing_if = "Option::is_none")]
    pub credentials_reference: Option<Box<crate::models::GlobalObjectKeySelector>>,
    #[serde(rename = "instanceProfileName", skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    /// A CIDR range that will be used to allow access to the node port range in the security group to. Only applies if the security group is generated by KKP and not preexisting. If NodePortsAllowedIPRange nor NodePortsAllowedIPRanges is set, the node port range can be accessed from anywhere.
    #[serde(rename = "nodePortsAllowedIPRange", skip_serializing_if = "Option::is_none")]
    pub node_ports_allowed_ip_range: Option<String>,
    #[serde(rename = "nodePortsAllowedIPRanges", skip_serializing_if = "Option::is_none")]
    pub node_ports_allowed_ip_ranges: Option<Box<crate::models::NetworkRanges>>,
    /// The IAM role, the control plane will use. The control plane will perform an assume-role
    #[serde(rename = "roleARN", skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "routeTableID", skip_serializing_if = "Option::is_none")]
    pub route_table_id: Option<String>,
    #[serde(rename = "secretAccessKey", skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "securityGroupID", skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    #[serde(rename = "vpcID", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl AwsCloudSpec {
    pub fn new() -> AwsCloudSpec {
        AwsCloudSpec {
            access_key_id: None,
            assume_role_arn: None,
            assume_role_external_id: None,
            credentials_reference: None,
            instance_profile_name: None,
            node_ports_allowed_ip_range: None,
            node_ports_allowed_ip_ranges: None,
            role_arn: None,
            route_table_id: None,
            secret_access_key: None,
            security_group_id: None,
            vpc_id: None,
        }
    }
}


