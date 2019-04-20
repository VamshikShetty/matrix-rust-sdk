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
pub struct Model200Register {
    /// An access token for the account. This access token can then be used to authorize other requests. Required if the ``inhibit_login`` option is false.
    #[serde(rename = "access_token",skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,
    /// ID of the registered device. Will be the same as the corresponding parameter in the request, if one was specified. Required if the ``inhibit_login`` option is false.
    #[serde(rename = "device_id",skip_serializing_if="Option::is_none")]
    pub device_id: Option<String>,
    /// The fully-qualified Matrix user ID (MXID) that has been registered.  Any user ID returned by this API must conform to the grammar given in the `Matrix specification <https://matrix.org/docs/spec/appendices.html#user-identifiers>`_.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// The server_name of the homeserver on which the account has been registered.  **Deprecated**. Clients should extract the server_name from ``user_id`` (by splitting at the first colon) if they require it. Note also that ``homeserver`` is not spelt this way.
    #[serde(rename = "home_server",skip_serializing_if="Option::is_none")]
    pub home_server: Option<String>,
}

impl Model200Register {
    pub fn new(user_id: String) -> Model200Register {
        Model200Register {
            access_token: None,
            device_id: None,
            user_id: user_id,
            home_server: None,
        }
    }
}
