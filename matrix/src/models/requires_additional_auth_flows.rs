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
pub struct RequiresAdditionalAuthFlows {
    /// The login type of each of the stages required to complete this authentication flow
    #[serde(rename = "stages")]
    pub stages: Vec<String>,
}

impl RequiresAdditionalAuthFlows {
    pub fn new(stages: Vec<String>) -> RequiresAdditionalAuthFlows {
        RequiresAdditionalAuthFlows {
            stages: stages,
        }
    }
}
