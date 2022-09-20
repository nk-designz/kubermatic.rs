/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SecretReference : SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace +structType=atomic



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretReference {
    /// name is unique within a namespace to reference a secret resource. +optional
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique. +optional
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl SecretReference {
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace +structType=atomic
    pub fn new() -> SecretReference {
        SecretReference {
            name: None,
            namespace: None,
        }
    }
}


