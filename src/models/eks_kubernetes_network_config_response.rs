/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EksKubernetesNetworkConfigResponse : The Kubernetes network configuration for the cluster. The response contains a value for serviceIpv6Cidr or serviceIpv4Cidr, but not both.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EksKubernetesNetworkConfigResponse {
    /// The IP family used to assign Kubernetes pod and service IP addresses. The IP family is always ipv4, unless you have a 1.21 or later cluster running version 1.10.1 or later of the Amazon VPC CNI add-on and specified ipv6 when you created the cluster.
    #[serde(rename = "ipFamily", skip_serializing_if = "Option::is_none")]
    pub ip_family: Option<String>,
    /// The CIDR block that Kubernetes pod and service IP addresses are assigned from. Kubernetes assigns addresses from an IPv4 CIDR block assigned to a subnet that the node is in. If you didn't specify a CIDR block when you created the cluster, then Kubernetes assigns addresses from either the 10.100.0.0/16 or 172.20.0.0/16 CIDR blocks. If this was specified, then it was specified when the cluster was created and it can't be changed.
    #[serde(rename = "serviceIpv4Cidr", skip_serializing_if = "Option::is_none")]
    pub service_ipv4_cidr: Option<String>,
    /// The CIDR block that Kubernetes pod and service IP addresses are assigned from if you created a 1.21 or later cluster with version 1.10.1 or later of the Amazon VPC CNI add-on and specified ipv6 for ipFamily when you created the cluster. Kubernetes assigns service addresses from the unique local address range (fc00::/7) because you can't specify a custom IPv6 CIDR block when you create the cluster.
    #[serde(rename = "serviceIpv6Cidr", skip_serializing_if = "Option::is_none")]
    pub service_ipv6_cidr: Option<String>,
}

impl EksKubernetesNetworkConfigResponse {
    /// The Kubernetes network configuration for the cluster. The response contains a value for serviceIpv6Cidr or serviceIpv4Cidr, but not both.
    pub fn new() -> EksKubernetesNetworkConfigResponse {
        EksKubernetesNetworkConfigResponse {
            ip_family: None,
            service_ipv4_cidr: None,
            service_ipv6_cidr: None,
        }
    }
}


