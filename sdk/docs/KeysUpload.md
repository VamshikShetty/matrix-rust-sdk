# KeysUpload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_keys** | [***::models::DeviceKeys**](device_keys.md) | Identity keys for the device. May be absent if no new identity keys are required. | [optional] 
**one_time_keys** | [***Value**](.md) | One-time public keys for `pre-key` messages. The names of the properties should be in the format `<algorithm>:<key_id>`. The format of the key is determined by the key algorithm. May be absent if no new one-time keys are required. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


