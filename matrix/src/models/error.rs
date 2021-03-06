/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Error : A Matrix-level Error

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    /// An error code.
    #[serde(rename = "errcode")]
    pub errcode: String,
    /// A human-readable error message.
    #[serde(rename = "error",skip_serializing_if="Option::is_none")]
    pub error: Option<String>,
}

impl Error {
    /// A Matrix-level Error
    pub fn new(errcode: String) -> Error {
        Error {
            errcode: errcode,
            error: None,
        }
    }
}
