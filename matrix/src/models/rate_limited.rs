/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RateLimited : The rate limit was reached for this request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimited {
    /// The M_LIMIT_EXCEEDED error code
    #[serde(rename = "errcode")]
    pub errcode: String,
    /// A human-readable error message.
    #[serde(rename = "error",skip_serializing_if="Option::is_none")]
    pub error: Option<String>,
    /// The amount of time in milliseconds the client should wait before trying the request again.
    #[serde(rename = "retry_after_ms",skip_serializing_if="Option::is_none")]
    pub retry_after_ms: Option<i32>,
}

impl RateLimited {
    /// The rate limit was reached for this request
    pub fn new(errcode: String) -> RateLimited {
        RateLimited {
            errcode: errcode,
            error: None,
            retry_after_ms: None,
        }
    }
}
