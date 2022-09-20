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
pub struct EksMachineDeploymentCloudSpec {
    /// The AMI type for your node group. GPU instance types should use the AL2_x86_64_GPU AMI type. Non-GPU instances should use the AL2_x86_64 AMI type. Arm instances should use the AL2_ARM_64 AMI type. All types use the Amazon EKS optimized Amazon Linux 2 AMI. If you specify launchTemplate, and your launch template uses a custom AMI, then don't specify amiType, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see Launch template support (https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html) in the Amazon EKS User Guide.
    #[serde(rename = "amiType", skip_serializing_if = "Option::is_none")]
    pub ami_type: Option<String>,
    /// The capacity type for your node group. Possible values ON_DEMAND | SPOT
    #[serde(rename = "capacityType", skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
    /// The Unix epoch timestamp in seconds for when the managed node group was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The root device disk size (in GiB) for your node group instances. The default disk size is 20 GiB. If you specify launchTemplate, then don't specify diskSize, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see Launch template support (https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html) in the Amazon EKS User Guide.
    #[serde(rename = "diskSize", skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    /// Specify the instance types for a node group. If you specify a GPU instance type, be sure to specify AL2_x86_64_GPU with the amiType parameter. If you specify launchTemplate, then you can specify zero or one instance type in your launch template or you can specify 0-20 instance types for instanceTypes. If however, you specify an instance type in your launch template and specify any instanceTypes, the node group deployment will fail. If you don't specify an instance type in a launch template or for instanceTypes, then t3.medium is used, by default. If you specify Spot for capacityType, then we recommend specifying multiple values for instanceTypes. For more information, see Managed node group capacity types (https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html#managed-node-group-capacity-types) and Launch template support (https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html) in the Amazon EKS User Guide.
    #[serde(rename = "instanceTypes", skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// The Kubernetes labels to be applied to the nodes in the node group when they are created.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// The Amazon Resource Name (ARN) of the IAM role to associate with your node group. The Amazon EKS worker node kubelet daemon makes calls to AWS APIs on your behalf. Nodes receive permissions for these API calls through an IAM instance profile and associated policies. Before you can launch nodes and register them into a cluster, you must create an IAM role for those nodes to use when they are launched. For more information, see Amazon EKS node IAM role (https://docs.aws.amazon.com/eks/latest/userguide/worker_node_IAM_role.html) in the Amazon EKS User Guide . If you specify launchTemplate, then don't specify IamInstanceProfile (https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_IamInstanceProfile.html) in your launch template, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see Launch template support (https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html) in the Amazon EKS User Guide.  NodeRole is a required field
    #[serde(rename = "nodeRole", skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,
    #[serde(rename = "scalingConfig", skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<Box<crate::models::EksNodegroupScalingConfig>>,
    /// The subnets to use for the Auto Scaling group that is created for your node group. These subnets must have the tag key kubernetes.io/cluster/CLUSTER_NAME with a value of shared, where CLUSTER_NAME is replaced with the name of your cluster. If you specify launchTemplate, then don't specify SubnetId (https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateNetworkInterface.html) in your launch template, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see Launch template support (https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html) in the Amazon EKS User Guide.  Subnets is a required field
    #[serde(rename = "subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// The metadata applied to the node group to assist with categorization and organization. Each tag consists of a key and an optional value. You define both. Node group tags do not propagate to any other resources associated with the node group, such as the Amazon EC2 instances or subnets.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// The Kubernetes version to use for your managed nodes. By default, the Kubernetes version of the cluster is used, and this is the only accepted specified value. If you specify launchTemplate, and your launch template uses a custom AMI, then don't specify version, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see Launch template support (https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html) in the Amazon EKS User Guide.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl EksMachineDeploymentCloudSpec {
    pub fn new() -> EksMachineDeploymentCloudSpec {
        EksMachineDeploymentCloudSpec {
            ami_type: None,
            capacity_type: None,
            created_at: None,
            disk_size: None,
            instance_types: None,
            labels: None,
            node_role: None,
            scaling_config: None,
            subnets: None,
            tags: None,
            version: None,
        }
    }
}

