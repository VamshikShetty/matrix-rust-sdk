use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        return Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

use super::models::*;

mod end_to_end_encryption_api;
pub use self::end_to_end_encryption_api::{ EndToEndEncryptionApi, EndToEndEncryptionApiClient };
mod room_participation_api;
pub use self::room_participation_api::{ RoomParticipationApi, RoomParticipationApiClient };
mod send_to_device_messaging_api;
pub use self::send_to_device_messaging_api::{ SendToDeviceMessagingApi, SendToDeviceMessagingApiClient };
mod session_management_api;
pub use self::session_management_api::{ SessionManagementApi, SessionManagementApiClient };

pub mod configuration;
pub mod client;
