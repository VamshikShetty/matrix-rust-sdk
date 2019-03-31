# LoginRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | The login type being used. | 
**identifier** | [***::models::UserIdentifier**](user_identifier.md) |  | [optional] 
**medium** | **String** | When logging in using a third party identifier, the medium of the identifier. Must be 'email'.  Deprecated in favour of ``identifier``. | [optional] 
**address** | **String** | Third party identifier for the user.  Deprecated in favour of ``identifier``. | [optional] 
**password** | **String** | Required when ``type`` is ``m.login.password``. The user's password. | [optional] 
**token** | **String** | Required when ``type`` is ``m.login.token``. Part of `Token-based`_ login. | [optional] 
**device_id** | **String** | ID of the client device. If this does not correspond to a known client device, a new device will be created. The server will auto-generate a device_id if this is not specified. | [optional] 
**initial_device_display_name** | **String** | A display name to assign to the newly-created device. Ignored if ``device_id`` corresponds to a known device. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


