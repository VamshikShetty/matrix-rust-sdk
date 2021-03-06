/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InviteSigned : A block of content which has been signed, which servers can use to verify the event. Clients should ignore this.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteSigned {
    /// The invited matrix user ID. Must be equal to the user_id property of the event.
    #[serde(rename = "mxid",skip_serializing_if="Option::is_none")]
    pub mxid: Option<String>,
    /// A single signature from the verifying server, in the format specified by the Signing Events section of the server-server API.
    #[serde(rename = "signatures",skip_serializing_if="Option::is_none")]
    pub signatures: Option<Value>,
    /// The token property of the containing third_party_invite object.
    #[serde(rename = "token",skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

impl InviteSigned {
    /// A block of content which has been signed, which servers can use to verify the event. Clients should ignore this.
    pub fn new() -> InviteSigned {
        InviteSigned {
            mxid: None,
            signatures: None,
            token: None,
        }
    }
}
