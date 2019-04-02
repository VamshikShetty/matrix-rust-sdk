# SyncResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_batch** | **String** | The batch token to supply in the ``since`` param of the next `/sync` request. | [optional] 
**rooms** | [***::models::SyncRooms**](sync_rooms.md) |  | [optional] 
**presence** | [***::models::SyncPresence**](sync_presence.md) |  | [optional] 
**account_data** | [***::models::SyncAccountData**](sync_account_data.md) |  | [optional] 
**to_device** | [***Value**](.md) | Information on the send-to-device messages for the client device, as defined in |send_to_device_sync|. | [optional] 
**device_lists** | [***::models::DeviceLists**](DeviceLists.md) |  | [optional] 
**device_one_time_keys_count** | **::std::collections::HashMap<String, i32>** |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


