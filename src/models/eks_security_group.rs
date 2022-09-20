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
pub struct EksSecurityGroup {
    /// The ID of the security group.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// [VPC only] The ID of the VPC for the security group.
    #[serde(rename = "vpcId", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl EksSecurityGroup {
    pub fn new() -> EksSecurityGroup {
        EksSecurityGroup {
            group_id: None,
            vpc_id: None,
        }
    }
}


