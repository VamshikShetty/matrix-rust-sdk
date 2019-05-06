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

pub struct DeviceManagementApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DeviceManagementApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DeviceManagementApiClient {
        DeviceManagementApiClient {
            configuration: configuration,
        }
    }
}

pub trait DeviceManagementApi {
    fn delete_device(&self, device_id: &str, device_delete_req: ::models::DeviceDeleteReq) -> Result<Value, Error>;
    fn delete_devices(&self, delete_devices_request_body: ::models::DeleteDevicesRequestBody) -> Result<Value, Error>;
    fn get_device(&self, device_id: &str) -> Result<::models::Device, Error>;
    fn get_devices(&self, ) -> Result<::models::ListOfDevice, Error>;
    fn update_device(&self, device_id: &str, display_name: ::models::DisplayName) -> Result<Value, Error>;
}

impl DeviceManagementApi for DeviceManagementApiClient {
    fn delete_device(&self, device_id: &str, device_delete_req: ::models::DeviceDeleteReq) -> Result<Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/devices/{deviceId}", configuration.base_path, deviceId=urlencode(device_id));
        let mut req_builder = client.delete(uri_str.as_str());

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
        req_builder = req_builder.json(&device_delete_req);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_devices(&self, delete_devices_request_body: ::models::DeleteDevicesRequestBody) -> Result<Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/delete_devices", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

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
        req_builder = req_builder.json(&delete_devices_request_body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_device(&self, device_id: &str) -> Result<::models::Device, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/devices/{deviceId}", configuration.base_path, deviceId=urlencode(device_id));
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

    fn get_devices(&self, ) -> Result<::models::ListOfDevice, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/devices", configuration.base_path);
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

    fn update_device(&self, device_id: &str, display_name: ::models::DisplayName) -> Result<Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/client/r0/devices/{deviceId}", configuration.base_path, deviceId=urlencode(device_id));
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
        req_builder = req_builder.json(&display_name);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
