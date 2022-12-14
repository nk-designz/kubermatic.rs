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
pub struct GitReference {
    /// Branch to checkout. Only the last commit of the branch will be checkout in order to reduce the amount of data to download. +optional
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Commit SHA in a Branch to checkout.  It must be used in conjunction with branch field.
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    /// Tag to check out. It can not be used in conjunction with commit or branch. +kubebuilder:validation:Type=string +optional
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl GitReference {
    pub fn new() -> GitReference {
        GitReference {
            branch: None,
            commit: None,
            tag: None,
        }
    }
}


