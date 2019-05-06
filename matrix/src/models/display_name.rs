/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayName {
    /// The new display name for this device. If not given, the display name is unchanged.
    #[serde(rename = "display_name",skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
}

impl DisplayName {
    pub fn new() -> DisplayName {
        DisplayName {
            display_name: None,
        }
    }
}