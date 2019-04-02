/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UnreadNotifications : Counts of unread notifications for this room

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnreadNotifications {
    /// The number of unread notifications for this room with the highlight flag set
    #[serde(rename = "highlight_count",skip_serializing_if="Option::is_none")]
    pub highlight_count: Option<i32>,
    /// The total number of unread notifications for this room
    #[serde(rename = "notification_count",skip_serializing_if="Option::is_none")]
    pub notification_count: Option<i32>,
}

impl UnreadNotifications {
    /// Counts of unread notifications for this room
    pub fn new() -> UnreadNotifications {
        UnreadNotifications {
            highlight_count: None,
            notification_count: None,
        }
    }
}
