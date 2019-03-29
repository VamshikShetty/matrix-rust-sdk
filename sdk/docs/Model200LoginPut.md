# Model200LoginPut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The fully-qualified Matrix ID that has been registered. | [optional] 
**access_token** | **String** | An access token for the account. This access token can then be used to authorize other requests. | [optional] 
**home_server** | **String** | The server_name of the homeserver on which the account has been registered. **Deprecated**. Clients should extract the server_name from ``user_id`` (by splitting at the first colon) if they require it. Note also that ``homeserver`` is not spelt this way. | [optional] 
**device_id** | **String** | ID of the logged-in device. Will be the same as the corresponding parameter in the request, if one was specified.  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


