use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    configuration: Rc<Configuration>,
    device_management_api: Box<::apis::DeviceManagementApi>,
    end_to_end_encryption_api: Box<::apis::EndToEndEncryptionApi>,
    media_api: Box<::apis::MediaApi>,
    room_creation_api: Box<::apis::RoomCreationApi>,
    room_directory_api: Box<::apis::RoomDirectoryApi>,
    room_membership_api: Box<::apis::RoomMembershipApi>,
    room_participation_api: Box<::apis::RoomParticipationApi>,
    send_to_device_messaging_api: Box<::apis::SendToDeviceMessagingApi>,
    session_management_api: Box<::apis::SessionManagementApi>,
    user_data_api: Box<::apis::UserDataApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            device_management_api: Box::new(::apis::DeviceManagementApiClient::new(rc.clone())),
            end_to_end_encryption_api: Box::new(::apis::EndToEndEncryptionApiClient::new(rc.clone())),
            media_api: Box::new(::apis::MediaApiClient::new(rc.clone())),
            room_creation_api: Box::new(::apis::RoomCreationApiClient::new(rc.clone())),
            room_directory_api: Box::new(::apis::RoomDirectoryApiClient::new(rc.clone())),
            room_membership_api: Box::new(::apis::RoomMembershipApiClient::new(rc.clone())),
            room_participation_api: Box::new(::apis::RoomParticipationApiClient::new(rc.clone())),
            send_to_device_messaging_api: Box::new(::apis::SendToDeviceMessagingApiClient::new(rc.clone())),
            session_management_api: Box::new(::apis::SessionManagementApiClient::new(rc.clone())),
            user_data_api: Box::new(::apis::UserDataApiClient::new(rc.clone())),
        }
    }

    pub fn device_management_api(&self) -> &::apis::DeviceManagementApi{
        self.device_management_api.as_ref()
    }

    pub fn end_to_end_encryption_api(&self) -> &::apis::EndToEndEncryptionApi{
        self.end_to_end_encryption_api.as_ref()
    }

    pub fn media_api(&self) -> &::apis::MediaApi{
        self.media_api.as_ref()
    }

    pub fn room_creation_api(&self) -> &::apis::RoomCreationApi{
        self.room_creation_api.as_ref()
    }

    pub fn room_directory_api(&self) -> &::apis::RoomDirectoryApi{
        self.room_directory_api.as_ref()
    }

    pub fn room_membership_api(&self) -> &::apis::RoomMembershipApi{
        self.room_membership_api.as_ref()
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

    pub fn user_data_api(&self) -> &::apis::UserDataApi{
        self.user_data_api.as_ref()
    }

}
