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
pub struct Model200SendEventTxnid {
    /// A unique identifier for the event.
    #[serde(rename = "event_id",skip_serializing_if="Option::is_none")]
    pub event_id: Option<String>,
}

impl Model200SendEventTxnid {
    pub fn new() -> Model200SendEventTxnid {
        Model200SendEventTxnid {
            event_id: None,
        }
    }
}
