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
    fn get_room_events(&self, room_id: &str, from: &str, dir: &str, to: &str, limit: i32, filter: &str) -> Result<::models::RoomidMessages, Error>;
    fn get_room_state_by_type(&self, room_id: &str, event_type: &str) -> Result<Value, Error>;
    fn redact_event(&self, room_id: &str, event_id: &str, txn_id: &str, redact_even_id_txn_id: ::models::RedactEvenIdTxnId) -> Result<::models::EventId, Error>;
    fn send_event_txnid(&self, room_id: &str, event_type: &str, txn_id: &str, body: Value) -> Result<::models::EventId, Error>;
    fn set_room_state(&self, room_id: &str, event_type: &str, redact_even_id_txn_id: ::models::RedactEvenIdTxnId) -> Result<::models::Model200StateEventType, Error>;
    fn sync(&self, filter: &str, since: &str, full_state: bool, set_presence: &str, timeout: i32) -> Result<::models::SyncResponse, Error>;
}

impl RoomParticipationApi for RoomParticipationApiClient {
    fn get_room_events(&self, room_id: &str, from: &str, dir: &str, to: &str, limit: i32, filter: &str) -> Result<::models::RoomidMessages, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/rooms/{roomId}/messages", configuration.base_path, roomId=urlencode(room_id));
        let mut req_builder = client.get(uri_str.as_str());

        let query_from = &from.to_string();
        if ! query_from.is_empty() {
            req_builder = req_builder.query(&[("from", query_from)]);
        }
        let query_to = &to.to_string();
        if ! query_to.is_empty() {
            req_builder = req_builder.query(&[("to", query_to)]);
        }
        let query_dir = &dir.to_string();
        if ! query_dir.is_empty() {
            req_builder = req_builder.query(&[("dir", query_dir)]);
        }
        let query_limit = &limit.to_string();
        if ! query_limit.is_empty() {
            req_builder = req_builder.query(&[("limit", query_limit)]);
        }
        let query_filter = &filter.to_string();
        if ! query_filter.is_empty() {
            req_builder = req_builder.query(&[("filter", query_filter)]);
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

    fn get_room_state_by_type(&self, room_id: &str, event_type: &str) -> Result<Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/rooms/{roomId}/state/{eventType}", configuration.base_path, roomId=urlencode(room_id), eventType=urlencode(event_type));
        let mut req_builder = client.get(uri_str.as_str());

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

    fn redact_event(&self, room_id: &str, event_id: &str, txn_id: &str, redact_even_id_txn_id: ::models::RedactEvenIdTxnId) -> Result<::models::EventId, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/rooms/{roomId}/redact/{eventId}/{txnId}", configuration.base_path, roomId=urlencode(room_id), eventId=urlencode(event_id), txnId=urlencode(txn_id));
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
        req_builder = req_builder.json(&redact_even_id_txn_id);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn send_event_txnid(&self, room_id: &str, event_type: &str, txn_id: &str, body: Value) -> Result<::models::EventId, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/rooms/{roomId}/send/{eventType}/{txnId}", configuration.base_path, roomId=urlencode(room_id), eventType=urlencode(event_type), txnId=urlencode(txn_id));
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

    fn set_room_state(&self, room_id: &str, event_type: &str, redact_even_id_txn_id: ::models::RedactEvenIdTxnId) -> Result<::models::Model200StateEventType, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/rooms/{roomId}/state/{eventType}", configuration.base_path, roomId=urlencode(room_id), eventType=urlencode(event_type));
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
        req_builder = req_builder.json(&redact_even_id_txn_id);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn sync(&self, filter: &str, since: &str, full_state: bool, set_presence: &str, timeout: i32) -> Result<::models::SyncResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/sync", configuration.base_path);
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
