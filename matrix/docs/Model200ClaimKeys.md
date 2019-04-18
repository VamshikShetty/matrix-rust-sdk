# Model200ClaimKeys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failures** | [**::std::collections::HashMap<String, Value>**](Value.md) | If any remote homeservers could not be reached, they are recorded here. The names of the properties are the names of the unreachable servers.  If the homeserver could be reached, but the user or device was unknown, no failure is recorded. Instead, the corresponding user or device is missing from the ``one_time_keys`` result. | [optional] 
**one_time_keys** | [**::std::collections::HashMap<String, ::std::collections::HashMap<String, Value>>**](map.md) | One-time keys for the queried devices. A map from user ID, to a map from devices to a map from ``<algorithm>:<key_id>`` to the key object. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


