/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeviceKeys : Device identity keys

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceKeys {
    /// The ID of the user the device belongs to. Must match the user ID used when logging in.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// The ID of the device these keys belong to. Must match the device ID used when logging in.
    #[serde(rename = "device_id")]
    pub device_id: String,
    /// The encryption algorithms supported by this device.
    #[serde(rename = "algorithms")]
    pub algorithms: Vec<String>,
    /// Public identity keys. The names of the properties should be in the format `<algorithm>:<device_id>`. The keys themselves should be encoded as specified by the key algorithm.
    #[serde(rename = "keys")]
    pub keys: ::std::collections::HashMap<String, String>,
    /// Signatures for the device key object. A map from user ID, to a map from `<algorithm>:<device_id>` to the signature. The signature is calculated using the process described at `Signing JSON`.
    #[serde(rename = "signatures")]
    pub signatures: ::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>,
    #[serde(rename = "unsigned",skip_serializing_if="Option::is_none")]
    pub unsigned: Option<::models::UnsignedDeviceInfo>,
}

impl DeviceKeys {
    /// Device identity keys
    pub fn new(user_id: String, device_id: String, algorithms: Vec<String>, keys: ::std::collections::HashMap<String, String>, signatures: ::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>) -> DeviceKeys {
        DeviceKeys {
            user_id: user_id,
            device_id: device_id,
            algorithms: algorithms,
            keys: keys,
            signatures: signatures,
            unsigned: None,
        }
    }
}
