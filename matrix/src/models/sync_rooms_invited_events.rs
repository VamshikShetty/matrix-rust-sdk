/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SyncRoomsInvitedEvents : The StrippedState events that form the invite state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRoomsInvitedEvents {
    /// Required. The content for the event.
    #[serde(rename = "content",skip_serializing_if="Option::is_none")]
    pub content: Option<Value>,
    /// Required. The state_key for the event.
    #[serde(rename = "state_key",skip_serializing_if="Option::is_none")]
    pub state_key: Option<String>,
    /// Required. The type for the event.
    #[serde(rename = "type",skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    /// Required. The sender for the event.
    #[serde(rename = "sender",skip_serializing_if="Option::is_none")]
    pub sender: Option<String>,
}

impl SyncRoomsInvitedEvents {
    /// The StrippedState events that form the invite state.
    pub fn new() -> SyncRoomsInvitedEvents {
        SyncRoomsInvitedEvents {
            content: None,
            state_key: None,
            _type: None,
            sender: None,
        }
    }
}