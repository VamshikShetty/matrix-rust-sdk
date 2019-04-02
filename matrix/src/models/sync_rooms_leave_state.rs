/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SyncRoomsLeaveState : The state updates for the room up to the start of the timeline.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRoomsLeaveState {
    #[serde(rename = "events",skip_serializing_if="Option::is_none")]
    pub events: Option<::models::ListOfEvent>,
}

impl SyncRoomsLeaveState {
    /// The state updates for the room up to the start of the timeline.
    pub fn new() -> SyncRoomsLeaveState {
        SyncRoomsLeaveState {
            events: None,
        }
    }
}
