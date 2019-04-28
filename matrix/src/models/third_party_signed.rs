/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ThirdPartySigned : A signature of an ``m.third_party_invite`` token to prove that this user owns a third party identity which has been invited to the room.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThirdPartySigned {
    #[serde(rename = "signed")]
    pub signed: ::models::JoinSigned,
}

impl ThirdPartySigned {
    /// A signature of an ``m.third_party_invite`` token to prove that this user owns a third party identity which has been invited to the room.
    pub fn new(signed: ::models::JoinSigned) -> ThirdPartySigned {
        ThirdPartySigned {
            signed: signed,
        }
    }
}