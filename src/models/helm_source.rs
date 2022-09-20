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
pub struct HelmSource {
    /// Name of the Chart. +kubebuilder:validation:MinLength=1
    #[serde(rename = "chartName", skip_serializing_if = "Option::is_none")]
    pub chart_name: Option<String>,
    /// Version of the Chart. +kubebuilder:validation:MinLength=1
    #[serde(rename = "chartVersion", skip_serializing_if = "Option::is_none")]
    pub chart_version: Option<String>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Box<crate::models::HelmCredentials>>,
    /// URl of the helm repository. It can be an HTTP(s) repository (e.g. https://localhost/myrepo) or on OCI repository (e.g. oci://localhost:5000/myrepo). +kubebuilder:validation:Pattern=\"^(http|https|oci)://.+\"
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl HelmSource {
    pub fn new() -> HelmSource {
        HelmSource {
            chart_name: None,
            chart_version: None,
            credentials: None,
            url: None,
        }
    }
}


