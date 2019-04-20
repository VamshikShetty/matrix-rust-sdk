# RequiresAdditionalAuth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flows** | [**Vec<::models::RequiresAdditionalAuthFlows>**](requires_additional_auth_flows.md) | A list of the login flows supported by the server for this API. | 
**session** | **String** | This is a session identifier that the client must pass back to the home server, if one is provide, in subsequent attempts to authenticate in the same API call. | [optional] 
**completed** | **Vec<String>** | A list of the stages the client has completed successfully | [optional] 
**params** | [**::std::collections::HashMap<String, Value>**](Value.md) | Contains any information that the client will need to know in order to use a given type of authentication. For each login type presented, that type may be present as a key in this dictionary. For example, the public part of an OAuth client ID could be given here. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


