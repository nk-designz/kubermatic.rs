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
pub struct EtcdBackupConfigStatus {
    /// If the controller was configured with a cleanupContainer, CleanupRunning keeps track of the corresponding job
    #[serde(rename = "cleanupRunning", skip_serializing_if = "Option::is_none")]
    pub cleanup_running: Option<bool>,
    /// Conditions contains conditions of the EtcdBackupConfig
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::EtcdBackupConfigCondition>>,
    /// CurrentBackups tracks the creation and deletion progress if all backups managed by the EtcdBackupConfig
    #[serde(rename = "lastBackups", skip_serializing_if = "Option::is_none")]
    pub last_backups: Option<Vec<crate::models::BackupStatus>>,
}

impl EtcdBackupConfigStatus {
    pub fn new() -> EtcdBackupConfigStatus {
        EtcdBackupConfigStatus {
            cleanup_running: None,
            conditions: None,
            last_backups: None,
        }
    }
}


