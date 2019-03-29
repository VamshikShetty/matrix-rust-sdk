# QueryKeys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timeout** | **i32** | The time (in milliseconds) to wait when downloading keys from remote servers. 10 seconds is the recommended default. | [optional] 
**device_keys** | [**::std::collections::HashMap<String, Vec<String>>**](array.md) | The keys to be downloaded. A map from user ID, to a list of device IDs, or to an empty list to indicate all devices for the corresponding user. | 
**token** | **String** | If the client is fetching keys as a result of a device update received in a sync request, this should be the 'since' token of that sync request, or any later sync token. This allows the server to ensure its response contains the keys advertised by the notification in that sync. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


