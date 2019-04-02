/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Timeline : The timeline of messages and state changes in the room.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Timeline {
    /// `True` if the number of events returned was limited by the limit on the filter.
    #[serde(rename = "limited",skip_serializing_if="Option::is_none")]
    pub limited: Option<bool>,
    /// A token that can be supplied to the from parameter of the rooms /{roomId}/messages endpoint.
    #[serde(rename = "prev_batch",skip_serializing_if="Option::is_none")]
    pub prev_batch: Option<String>,
    #[serde(rename = "events",skip_serializing_if="Option::is_none")]
    pub events: Option<::models::TimelineEvents>,
}

impl Timeline {
    /// The timeline of messages and state changes in the room.
    pub fn new() -> Timeline {
        Timeline {
            limited: None,
            prev_batch: None,
            events: None,
        }
    }
}
