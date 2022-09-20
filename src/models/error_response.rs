/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ErrorResponse : ErrorResponse is the default representation of an error



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::ErrorDetails>>,
}

impl ErrorResponse {
    /// ErrorResponse is the default representation of an error
    pub fn new() -> ErrorResponse {
        ErrorResponse {
            error: None,
        }
    }
}


