/*
 * Matrix Client-Server Registration and Login API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;
use serde_json::Value;

use super::{Error, configuration, urlencode};

pub struct RoomParticipationApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl RoomParticipationApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> RoomParticipationApiClient {
        RoomParticipationApiClient {
            configuration: configuration,
        }
    }
}

pub trait RoomParticipationApi {
    fn send_event_txnid(&self, room_id: &str, event_type: &str, txn_id: &str, body: Value) -> Result<::models::Model200SendEventTxnid, Error>;
    fn sync(&self, filter: &str, since: &str, full_state: bool, set_presence: &str, timeout: i32) -> Result<::models::SyncResponse, Error>;
}

impl RoomParticipationApi for RoomParticipationApiClient {
    fn send_event_txnid(&self, room_id: &str, event_type: &str, txn_id: &str, body: Value) -> Result<::models::Model200SendEventTxnid, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rooms/{roomId}/send/{eventType}/{txnId}", configuration.base_path, roomId=urlencode(room_id), eventType=urlencode(event_type), txnId=urlencode(txn_id));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("access_token", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn sync(&self, filter: &str, since: &str, full_state: bool, set_presence: &str, timeout: i32) -> Result<::models::SyncResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/sync", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        let query_filter = &filter.to_string();
        if ! query_filter.is_empty() {
            req_builder = req_builder.query(&[("filter", query_filter)]);
        }
        let query_since = &since.to_string();
        if ! query_since.is_empty() {
            req_builder = req_builder.query(&[("since", query_since)]);
        }
        let query_full_state = &full_state.to_string();
        if ! query_full_state.is_empty() {
            req_builder = req_builder.query(&[("full_state", query_full_state)]);
        }
        let query_set_presence = &set_presence.to_string();
        if ! query_set_presence.is_empty() {
            req_builder = req_builder.query(&[("set_presence", query_set_presence)]);
        }
        let query_timeout = &timeout.to_string();
        if ! query_timeout.is_empty() {
            req_builder = req_builder.query(&[("timeout", query_timeout)]);
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("access_token", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
