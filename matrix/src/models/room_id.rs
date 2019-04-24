/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RoomId : Information about the newly created room.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomId {
    /// The created room's ID.
    #[serde(rename = "room_id")]
    pub room_id: String,
}

impl RoomId {
    /// Information about the newly created room.
    pub fn new(room_id: String) -> RoomId {
        RoomId {
            room_id: room_id,
        }
    }
}
