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
pub struct Model200GetKeysChanges {
    /// The Matrix User IDs of all users who updated their device\\nidentity keys.
    #[serde(rename = "changed",skip_serializing_if="Option::is_none")]
    pub changed: Option<Vec<String>>,
    /// The Matrix User IDs of all users who may have left all\\nthe end-to-end encrypted rooms they previously shared\\nwith the user.
    #[serde(rename = "left",skip_serializing_if="Option::is_none")]
    pub left: Option<Vec<String>>,
}

impl Model200GetKeysChanges {
    pub fn new() -> Model200GetKeysChanges {
        Model200GetKeysChanges {
            changed: None,
            left: None,
        }
    }
}
