/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StrippedState : The StrippedState events that form the invite state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StrippedState {
    #[serde(rename = "content")]
    pub content: ::models::EventContent,
    /// Required. The state_key for the event.
    #[serde(rename = "state_key")]
    pub state_key: String,
    /// Required. The type for the event.
    #[serde(rename = "type")]
    pub _type: String,
    /// Required. The sender for the event.
    #[serde(rename = "sender")]
    pub sender: String,
}

impl StrippedState {
    /// The StrippedState events that form the invite state.
    pub fn new(content: ::models::EventContent, state_key: String, _type: String, sender: String) -> StrippedState {
        StrippedState {
            content: content,
            state_key: state_key,
            _type: _type,
            sender: sender,
        }
    }
}
