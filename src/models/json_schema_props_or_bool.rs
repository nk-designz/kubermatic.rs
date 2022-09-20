/*
 * Kubermatic Kubernetes Platform API
 *
 * This spec describes possible operations which can be made against the Kubermatic Kubernetes Platform API.
 *
 * The version of the OpenAPI document: 2.21
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonSchemaPropsOrBool : Defaults to true for the boolean property.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JsonSchemaPropsOrBool {
    #[serde(rename = "Allows", skip_serializing_if = "Option::is_none")]
    pub allows: Option<bool>,
    #[serde(rename = "Schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<crate::models::JsonSchemaProps>>,
}

impl JsonSchemaPropsOrBool {
    /// Defaults to true for the boolean property.
    pub fn new() -> JsonSchemaPropsOrBool {
        JsonSchemaPropsOrBool {
            allows: None,
            schema: None,
        }
    }
}


