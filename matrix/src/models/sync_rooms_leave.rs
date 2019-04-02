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
pub struct SyncRoomsLeave {
    #[serde(rename = "state",skip_serializing_if="Option::is_none")]
    pub state: Option<::models::SyncRoomsLeaveState>,
    #[serde(rename = "timeline",skip_serializing_if="Option::is_none")]
    pub timeline: Option<::models::Timeline>,
    #[serde(rename = "account_data",skip_serializing_if="Option::is_none")]
    pub account_data: Option<::models::SyncAccountData>,
}

impl SyncRoomsLeave {
    pub fn new() -> SyncRoomsLeave {
        SyncRoomsLeave {
            state: None,
            timeline: None,
            account_data: None,
        }
    }
}
