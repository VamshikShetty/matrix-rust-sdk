/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserIdentifier : Identification information for a user

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdentifier {
    /// The type of identification.  See `Identifier types`_ for supported values and additional property descriptions.
    #[serde(rename = "type")]
    pub _type: String,
    /// The fully qualified user ID or just local part of the user ID, to log in.  Deprecated in favour of ``identifier``.
    #[serde(rename = "user")]
    pub user: Option<String>,
}

impl UserIdentifier {
    /// Identification information for a user
    pub fn new(_type: String) -> UserIdentifier {
        UserIdentifier {
            _type: _type,
            user: None,
        }
    }
}