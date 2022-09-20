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
pub struct Awsvpc {
    /// The primary IPv4 CIDR block for the VPC.
    #[serde(rename = "cidrBlock", skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// Information about the IPv4 CIDR blocks associated with the VPC.
    #[serde(rename = "cidrBlockAssociationSet", skip_serializing_if = "Option::is_none")]
    pub cidr_block_association_set: Option<Vec<crate::models::AwsVpcCidrBlockAssociation>>,
    /// The ID of the set of DHCP options you've associated with the VPC (or default if the default options are associated with the VPC).
    #[serde(rename = "dhcpOptionsId", skip_serializing_if = "Option::is_none")]
    pub dhcp_options_id: Option<String>,
    /// The allowed tenancy of instances launched into the VPC.
    #[serde(rename = "instanceTenancy", skip_serializing_if = "Option::is_none")]
    pub instance_tenancy: Option<String>,
    /// Information about the IPv6 CIDR blocks associated with the VPC.
    #[serde(rename = "ipv6CidrBlockAssociationSet", skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block_association_set: Option<Vec<crate::models::AwsVpcIpv6CidrBlockAssociation>>,
    /// Indicates whether the VPC is the default VPC.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the AWS account that owns the VPC.
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// The current state of the VPC.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Any tags assigned to the VPC.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::AwsTag>>,
    /// The ID of the VPC.
    #[serde(rename = "vpcId", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl Awsvpc {
    pub fn new() -> Awsvpc {
        Awsvpc {
            cidr_block: None,
            cidr_block_association_set: None,
            dhcp_options_id: None,
            instance_tenancy: None,
            ipv6_cidr_block_association_set: None,
            is_default: None,
            name: None,
            owner_id: None,
            state: None,
            tags: None,
            vpc_id: None,
        }
    }
}


