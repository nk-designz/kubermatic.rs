/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PodDnsConfig : PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PodDnsConfig {
    /// A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed. +optional
    #[serde(rename = "nameservers", skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    /// A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy. +optional
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::PodDnsConfigOption>>,
    /// A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed. +optional
    #[serde(rename = "searches", skip_serializing_if = "Option::is_none")]
    pub searches: Option<Vec<String>>,
}

impl PodDnsConfig {
    /// PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.
    pub fn new() -> PodDnsConfig {
        PodDnsConfig {
            nameservers: None,
            options: None,
            searches: None,
        }
    }
}


