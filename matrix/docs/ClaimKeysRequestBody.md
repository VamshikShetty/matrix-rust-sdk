# ClaimKeysRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**one_time_keys** | [**::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>**](map.md) | The keys to be claimed. A map from user ID, to a map from device ID to algorithm name. | 
**timeout** | **i32** | The time (in milliseconds) to wait when downloading keys from remote servers. 10 seconds is the recommended default. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


