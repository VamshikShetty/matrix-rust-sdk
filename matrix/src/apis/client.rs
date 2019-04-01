use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    configuration: Rc<Configuration>,
    end_to_end_encryption_api: Box<::apis::EndToEndEncryptionApi>,
    room_participation_api: Box<::apis::RoomParticipationApi>,
    send_to_device_messaging_api: Box<::apis::SendToDeviceMessagingApi>,
    session_management_api: Box<::apis::SessionManagementApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            end_to_end_encryption_api: Box::new(::apis::EndToEndEncryptionApiClient::new(rc.clone())),
            room_participation_api: Box::new(::apis::RoomParticipationApiClient::new(rc.clone())),
            send_to_device_messaging_api: Box::new(::apis::SendToDeviceMessagingApiClient::new(rc.clone())),
            session_management_api: Box::new(::apis::SessionManagementApiClient::new(rc.clone())),
        }
    }

    pub fn end_to_end_encryption_api(&self) -> &::apis::EndToEndEncryptionApi{
        self.end_to_end_encryption_api.as_ref()
    }

    pub fn room_participation_api(&self) -> &::apis::RoomParticipationApi{
        self.room_participation_api.as_ref()
    }

    pub fn send_to_device_messaging_api(&self) -> &::apis::SendToDeviceMessagingApi{
        self.send_to_device_messaging_api.as_ref()
    }

    pub fn session_management_api(&self) -> &::apis::SessionManagementApi{
        self.session_management_api.as_ref()
    }

}
