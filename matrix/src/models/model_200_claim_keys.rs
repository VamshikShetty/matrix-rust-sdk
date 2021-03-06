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
pub struct Model200ClaimKeys {
    /// If any remote homeservers could not be reached, they are recorded here. The names of the properties are the names of the unreachable servers.  If the homeserver could be reached, but the user or device was unknown, no failure is recorded. Instead, the corresponding user or device is missing from the ``one_time_keys`` result.
    #[serde(rename = "failures",skip_serializing_if="Option::is_none")]
    pub failures: Option<::std::collections::HashMap<String, Value>>,
    /// One-time keys for the queried devices. A map from user ID, to a map from devices to a map from ``<algorithm>:<key_id>`` to the key object.
    #[serde(rename = "one_time_keys",skip_serializing_if="Option::is_none")]
    pub one_time_keys: Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, Value>>>,
}

impl Model200ClaimKeys {
    pub fn new() -> Model200ClaimKeys {
        Model200ClaimKeys {
            failures: None,
            one_time_keys: None,
        }
    }
}
