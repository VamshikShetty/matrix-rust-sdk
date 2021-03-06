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
pub struct TimelineEvent {
    /// Fields in this object will vary depending on the type of event. When interacting with the REST API, this is the HTTP body.
    #[serde(rename = "content",skip_serializing_if="Option::is_none")]
    pub content: Option<Value>,
    /// The type of event. This SHOULD be namespaced similar to Java package naming conventions e.g. `com.example.subdomain.event.type`
    #[serde(rename = "type",skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    /// The globally unique event identifier.
    #[serde(rename = "event_id",skip_serializing_if="Option::is_none")]
    pub event_id: Option<String>,
    /// Contains the fully-qualified ID of the user who sent this event.
    #[serde(rename = "sender",skip_serializing_if="Option::is_none")]
    pub sender: Option<String>,
    /// Timestamp in milliseconds on originating homeserver when this event was sent.
    #[serde(rename = "origin_server_ts",skip_serializing_if="Option::is_none")]
    pub origin_server_ts: Option<i32>,
    #[serde(rename = "unsigned",skip_serializing_if="Option::is_none")]
    pub unsigned: Option<::models::UnsignedData>,
}

impl TimelineEvent {
    pub fn new() -> TimelineEvent {
        TimelineEvent {
            content: None,
            _type: None,
            event_id: None,
            sender: None,
            origin_server_ts: None,
            unsigned: None,
        }
    }
}
