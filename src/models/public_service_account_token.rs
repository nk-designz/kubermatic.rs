/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PublicServiceAccountToken : PublicServiceAccountToken represent an API service account token without secret fields



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PublicServiceAccountToken {
    /// Annotations that can be added to the resource
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    /// CreationTimestamp is a timestamp representing the server time when this object was created.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// DeletionTimestamp is a timestamp representing the server time when this object was deleted.
    #[serde(rename = "deletionTimestamp", skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<String>,
    /// Expiry is a timestamp representing the time when this token will expire.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// ID unique value that identifies the resource generated by the server. Read-Only.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Invalidated indicates if the token must be regenerated
    #[serde(rename = "invalidated", skip_serializing_if = "Option::is_none")]
    pub invalidated: Option<bool>,
    /// Name represents human readable name for the resource
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PublicServiceAccountToken {
    /// PublicServiceAccountToken represent an API service account token without secret fields
    pub fn new() -> PublicServiceAccountToken {
        PublicServiceAccountToken {
            annotations: None,
            creation_timestamp: None,
            deletion_timestamp: None,
            expiry: None,
            id: None,
            invalidated: None,
            name: None,
        }
    }
}


