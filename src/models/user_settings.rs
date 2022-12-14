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
pub struct UserSettings {
    #[serde(rename = "collapseSidenav", skip_serializing_if = "Option::is_none")]
    pub collapse_sidenav: Option<bool>,
    #[serde(rename = "displayAllProjectsForAdmin", skip_serializing_if = "Option::is_none")]
    pub display_all_projects_for_admin: Option<bool>,
    #[serde(rename = "itemsPerPage", skip_serializing_if = "Option::is_none")]
    pub items_per_page: Option<i32>,
    #[serde(rename = "lastSeenChangelogVersion", skip_serializing_if = "Option::is_none")]
    pub last_seen_changelog_version: Option<String>,
    #[serde(rename = "selectProjectTableView", skip_serializing_if = "Option::is_none")]
    pub select_project_table_view: Option<bool>,
    #[serde(rename = "selectedProjectID", skip_serializing_if = "Option::is_none")]
    pub selected_project_id: Option<String>,
    #[serde(rename = "selectedTheme", skip_serializing_if = "Option::is_none")]
    pub selected_theme: Option<String>,
    #[serde(rename = "useClustersView", skip_serializing_if = "Option::is_none")]
    pub use_clusters_view: Option<bool>,
}

impl UserSettings {
    pub fn new() -> UserSettings {
        UserSettings {
            collapse_sidenav: None,
            display_all_projects_for_admin: None,
            items_per_page: None,
            last_seen_changelog_version: None,
            select_project_table_view: None,
            selected_project_id: None,
            selected_theme: None,
            use_clusters_view: None,
        }
    }
}


