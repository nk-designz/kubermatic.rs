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
pub struct ErrorDetails {
    /// The error code
    #[serde(rename = "code")]
    pub code: i64,
    /// Additional error messages
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
    /// The error message
    #[serde(rename = "message")]
    pub message: String,
}

impl ErrorDetails {
    pub fn new(code: i64, message: String) -> ErrorDetails {
        ErrorDetails {
            code,
            details: None,
            message,
        }
    }
}

