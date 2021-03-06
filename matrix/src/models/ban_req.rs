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
pub struct BanReq {
    /// The reason the user has been banned. This will be supplied as the ``reason`` on the target's updated `m.room.member`_ event.
    #[serde(rename = "reason",skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    /// The fully qualified user ID of the user being banned.
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl BanReq {
    pub fn new(user_id: String) -> BanReq {
        BanReq {
            reason: None,
            user_id: user_id,
        }
    }
}
