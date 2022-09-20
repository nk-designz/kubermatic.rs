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
pub struct ResourceQuotaStatus {
    #[serde(rename = "globalUsage", skip_serializing_if = "Option::is_none")]
    pub global_usage: Option<Box<crate::models::Quota>>,
    #[serde(rename = "localUsage", skip_serializing_if = "Option::is_none")]
    pub local_usage: Option<Box<crate::models::Quota>>,
}

impl ResourceQuotaStatus {
    pub fn new() -> ResourceQuotaStatus {
        ResourceQuotaStatus {
            global_usage: None,
            local_usage: None,
        }
    }
}

