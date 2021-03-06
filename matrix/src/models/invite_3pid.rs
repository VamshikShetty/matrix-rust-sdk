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
pub struct Invite3pid {
    /// The invitee's third party identifier.
    #[serde(rename = "address")]
    pub address: String,
    /// The kind of address being passed in the address field, for example ``email``.
    #[serde(rename = "medium")]
    pub medium: String,
    /// The hostname+port of the identity server which should be used for third party identifier lookups.
    #[serde(rename = "id_server")]
    pub id_server: String,
}

impl Invite3pid {
    pub fn new(address: String, medium: String, id_server: String) -> Invite3pid {
        Invite3pid {
            address: address,
            medium: medium,
            id_server: id_server,
        }
    }
}
