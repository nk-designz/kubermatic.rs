/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Admin : Admin represents admin user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Admin {
    /// Email address of the admin user
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// IsAdmin indicates admin role
    #[serde(rename = "isAdmin", skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    /// Name of the admin user
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Admin {
    /// Admin represents admin user
    pub fn new() -> Admin {
        Admin {
            email: None,
            is_admin: None,
            name: None,
        }
    }
}


