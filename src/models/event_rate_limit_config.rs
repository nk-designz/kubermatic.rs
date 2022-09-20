/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EventRateLimitConfig : More info: https://kubernetes.io/docs/reference/access-authn-authz/admission-controllers/#eventratelimit



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EventRateLimitConfig {
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Box<crate::models::EventRateLimitConfigItem>>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::EventRateLimitConfigItem>>,
    #[serde(rename = "sourceAndObject", skip_serializing_if = "Option::is_none")]
    pub source_and_object: Option<Box<crate::models::EventRateLimitConfigItem>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::EventRateLimitConfigItem>>,
}

impl EventRateLimitConfig {
    /// More info: https://kubernetes.io/docs/reference/access-authn-authz/admission-controllers/#eventratelimit
    pub fn new() -> EventRateLimitConfig {
        EventRateLimitConfig {
            namespace: None,
            server: None,
            source_and_object: None,
            user: None,
        }
    }
}


