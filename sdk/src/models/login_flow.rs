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
pub struct LoginFlow {
    /// The login type. This is supplied as the ``type`` when logging in.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

impl LoginFlow {
    pub fn new() -> LoginFlow {
        LoginFlow {
            _type: None,
        }
    }
}
