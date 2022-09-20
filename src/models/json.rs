/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Json : These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Json {
    #[serde(rename = "Raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<Vec<i32>>,
}

impl Json {
    /// These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
    pub fn new() -> Json {
        Json {
            raw: None,
        }
    }
}

