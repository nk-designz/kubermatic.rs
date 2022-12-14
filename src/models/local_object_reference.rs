/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LocalObjectReference : LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace. +structType=atomic



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LocalObjectReference {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid? +optional
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LocalObjectReference {
    /// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace. +structType=atomic
    pub fn new() -> LocalObjectReference {
        LocalObjectReference {
            name: None,
        }
    }
}


