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
pub struct VpcConfigRequest {
    /// Specify one or more security groups for the cross-account elastic network interfaces that Amazon EKS creates to use to allow communication between your nodes and the Kubernetes control plane. For more information, see Amazon EKS security group considerations (https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html) in the Amazon EKS User Guide .
    #[serde(rename = "securityGroupIds", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// Specify subnets for your Amazon EKS nodes. Amazon EKS creates cross-account elastic network interfaces in these subnets to allow communication between your nodes and the Kubernetes control plane.
    #[serde(rename = "subnetIds", skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// The VPC associated with your cluster.
    #[serde(rename = "vpcId", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl VpcConfigRequest {
    pub fn new() -> VpcConfigRequest {
        VpcConfigRequest {
            security_group_ids: None,
            subnet_ids: None,
            vpc_id: None,
        }
    }
}

