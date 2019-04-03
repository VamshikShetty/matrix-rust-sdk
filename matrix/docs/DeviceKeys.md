# DeviceKeys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user the device belongs to. Must match the user ID used when logging in. | 
**device_id** | **String** | The ID of the device these keys belong to. Must match the device ID used when logging in. | 
**algorithms** | **Vec<String>** | The encryption algorithms supported by this device. | 
**keys** | **::std::collections::HashMap<String, String>** | Public identity keys. The names of the properties should be in the format `<algorithm>:<device_id>`. The keys themselves should be encoded as specified by the key algorithm. | 
**signatures** | [**::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>**](map.md) | Signatures for the device key object. A map from user ID, to a map from `<algorithm>:<device_id>` to the signature. The signature is calculated using the process described at `Signing JSON`. | 
**unsigned** | [***::models::UnsignedDeviceInfo**](UnsignedDeviceInfo.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


