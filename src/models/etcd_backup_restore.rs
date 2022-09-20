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
pub struct EtcdBackupRestore {
    /// DefaultDestination marks the default destination that will be used for the default etcd backup config which is created for every user cluster. Has to correspond to a destination in Destinations. If removed, it removes the related default etcd backup configs.
    #[serde(rename = "defaultDestination", skip_serializing_if = "Option::is_none")]
    pub default_destination: Option<String>,
    /// Destinations stores all the possible destinations where the backups for the Seed can be stored. If not empty, it enables automatic backup and restore for the seed.
    #[serde(rename = "destinations", skip_serializing_if = "Option::is_none")]
    pub destinations: Option<::std::collections::HashMap<String, crate::models::BackupDestination>>,
}

impl EtcdBackupRestore {
    pub fn new() -> EtcdBackupRestore {
        EtcdBackupRestore {
            default_destination: None,
            destinations: None,
        }
    }
}


