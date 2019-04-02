/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SyncRooms : Updates to rooms.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRooms {
    /// The rooms that the user has joined.
    #[serde(rename = "join",skip_serializing_if="Option::is_none")]
    pub join: Option<::std::collections::HashMap<String, Value>>,
    #[serde(rename = "invited",skip_serializing_if="Option::is_none")]
    pub invited: Option<::models::SyncRoomsInvited>,
    #[serde(rename = "leave",skip_serializing_if="Option::is_none")]
    pub leave: Option<::models::SyncRoomsLeave>,
}

impl SyncRooms {
    /// Updates to rooms.
    pub fn new() -> SyncRooms {
        SyncRooms {
            join: None,
            invited: None,
            leave: None,
        }
    }
}
