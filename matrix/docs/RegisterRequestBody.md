# RegisterRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth** | [**::std::collections::HashMap<String, Value>**](Value.md) | Additional authentication information for the user-interactive authentication API. | [optional] 
**bind_email** | **bool** | If true, the server binds the email used for authentication to the Matrix ID with the identity server. | [optional] 
**device_id** | **String** | ID of the client device. If this does not correspond to a known client device, a new device will be created. The server will auto-generate a device_id if this is not specified. | [optional] 
**inhibit_login** | **bool** | If true, an ``access_token`` and ``device_id`` should not be returned from this call, therefore preventing an automatic login. Defaults to false. | [optional] 
**initial_device_display_name** | **String** | A display name to assign to the newly-created device. Ignored if ``device_id`` corresponds to a known device. | [optional] 
**password** | **String** | The desired password for the account. | [optional] 
**username** | **String** | The basis for the localpart of the desired Matrix ID. If omitted, the homeserver MUST generate a Matrix ID local part. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


