/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UnsignedDeviceInfo : Additional data added to the device key information by intermediate servers, and not covered by the signatures.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsignedDeviceInfo {
    /// The display name which the user set on the device.
    #[serde(rename = "device_display_name",skip_serializing_if="Option::is_none")]
    pub device_display_name: Option<String>,
}

impl UnsignedDeviceInfo {
    /// Additional data added to the device key information by intermediate servers, and not covered by the signatures.
    pub fn new() -> UnsignedDeviceInfo {
        UnsignedDeviceInfo {
            device_display_name: None,
        }
    }
}