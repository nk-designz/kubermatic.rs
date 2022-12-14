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
pub struct MeteringConfiguration {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// ReportConfigurations is a map of report configuration definitions.
    #[serde(rename = "reports", skip_serializing_if = "Option::is_none")]
    pub reports: Option<::std::collections::HashMap<String, crate::models::MeteringReportConfiguration>>,
    /// StorageClassName is the name of the storage class that the metering prometheus instance uses to store metric data for reporting.
    #[serde(rename = "storageClassName", skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,
    /// StorageSize is the size of the storage class. Default value is 100Gi.
    #[serde(rename = "storageSize", skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
}

impl MeteringConfiguration {
    pub fn new() -> MeteringConfiguration {
        MeteringConfiguration {
            enabled: None,
            reports: None,
            storage_class_name: None,
            storage_size: None,
        }
    }
}


