use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod addon_api;
pub mod admin_api;
pub mod aks_api;
pub mod alibaba_api;
pub mod allowedregistries_api;
pub mod allowedregistry_api;
pub mod anexia_api;
pub mod applications_api;
pub mod aws_api;
pub mod azure_api;
pub mod backupcredentials_api;
pub mod cniversion_api;
pub mod constraint_api;
pub mod constraints_api;
pub mod constrainttemplates_api;
pub mod credentials_api;
pub mod datacenter_api;
pub mod default_api;
pub mod digitalocean_api;
pub mod eks_api;
pub mod etcdbackupconfig_api;
pub mod etcdrestore_api;
pub mod feature_api;
pub mod gcp_api;
pub mod get_api;
pub mod gke_api;
pub mod hetzner_api;
pub mod ipampool_api;
pub mod kubevirt_api;
pub mod metering_api;
pub mod metric_api;
pub mod mlaadminsetting_api;
pub mod networkdefaults_api;
pub mod nutanix_api;
pub mod of_api;
pub mod openstack_api;
pub mod operatingsystemprofile_api;
pub mod packet_api;
pub mod preset_api;
pub mod project_api;
pub mod report_api;
pub mod reports_api;
pub mod resource_quota_api;
pub mod rulegroup_api;
pub mod seed_api;
pub mod serviceaccounts_api;
pub mod settings_api;
pub mod status_api;
pub mod tokens_api;
pub mod user_api;
pub mod users_api;
pub mod version_api;
pub mod versions_api;
pub mod vmwareclouddirector_api;
pub mod vsphere_api;

pub mod configuration;
