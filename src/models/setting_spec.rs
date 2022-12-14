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
pub struct SettingSpec {
    #[serde(rename = "cleanupOptions", skip_serializing_if = "Option::is_none")]
    pub cleanup_options: Option<Box<crate::models::CleanupOptions>>,
    #[serde(rename = "customLinks", skip_serializing_if = "Option::is_none")]
    pub custom_links: Option<crate::models::CustomLinks>,
    #[serde(rename = "defaultNodeCount", skip_serializing_if = "Option::is_none")]
    pub default_node_count: Option<i32>,
    #[serde(rename = "displayAPIDocs", skip_serializing_if = "Option::is_none")]
    pub display_api_docs: Option<bool>,
    #[serde(rename = "displayDemoInfo", skip_serializing_if = "Option::is_none")]
    pub display_demo_info: Option<bool>,
    #[serde(rename = "displayTermsOfService", skip_serializing_if = "Option::is_none")]
    pub display_terms_of_service: Option<bool>,
    #[serde(rename = "enableDashboard", skip_serializing_if = "Option::is_none")]
    pub enable_dashboard: Option<bool>,
    #[serde(rename = "enableExternalClusterImport", skip_serializing_if = "Option::is_none")]
    pub enable_external_cluster_import: Option<bool>,
    #[serde(rename = "enableOIDCKubeconfig", skip_serializing_if = "Option::is_none")]
    pub enable_oidc_kubeconfig: Option<bool>,
    #[serde(rename = "machineDeploymentVMResourceQuota", skip_serializing_if = "Option::is_none")]
    pub machine_deployment_vm_resource_quota: Option<Box<crate::models::MachineDeploymentVmResourceQuota>>,
    #[serde(rename = "mlaAlertmanagerPrefix", skip_serializing_if = "Option::is_none")]
    pub mla_alertmanager_prefix: Option<String>,
    #[serde(rename = "mlaGrafanaPrefix", skip_serializing_if = "Option::is_none")]
    pub mla_grafana_prefix: Option<String>,
    #[serde(rename = "mlaOptions", skip_serializing_if = "Option::is_none")]
    pub mla_options: Option<Box<crate::models::MlaOptions>>,
    #[serde(rename = "opaOptions", skip_serializing_if = "Option::is_none")]
    pub opa_options: Option<Box<crate::models::OpaOptions>>,
    #[serde(rename = "restrictProjectCreation", skip_serializing_if = "Option::is_none")]
    pub restrict_project_creation: Option<bool>,
    #[serde(rename = "userProjectsLimit", skip_serializing_if = "Option::is_none")]
    pub user_projects_limit: Option<i64>,
}

impl SettingSpec {
    pub fn new() -> SettingSpec {
        SettingSpec {
            cleanup_options: None,
            custom_links: None,
            default_node_count: None,
            display_api_docs: None,
            display_demo_info: None,
            display_terms_of_service: None,
            enable_dashboard: None,
            enable_external_cluster_import: None,
            enable_oidc_kubeconfig: None,
            machine_deployment_vm_resource_quota: None,
            mla_alertmanager_prefix: None,
            mla_grafana_prefix: None,
            mla_options: None,
            opa_options: None,
            restrict_project_creation: None,
            user_projects_limit: None,
        }
    }
}


