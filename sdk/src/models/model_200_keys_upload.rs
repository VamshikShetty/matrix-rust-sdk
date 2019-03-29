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
pub struct Model200KeysUpload {
    /// For each key algorithm, the number of unclaimed one-time keys of that type currently held on the server for this device.
    #[serde(rename = "one_time_key_counts")]
    pub one_time_key_counts: ::std::collections::HashMap<String, i32>,
}

impl Model200KeysUpload {
    pub fn new(one_time_key_counts: ::std::collections::HashMap<String, i32>) -> Model200KeysUpload {
        Model200KeysUpload {
            one_time_key_counts: one_time_key_counts,
        }
    }
}
