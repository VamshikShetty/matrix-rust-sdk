# Stateevent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | [***Value**](.md) | Fields in this object will vary depending on the type of event. When interacting with the REST API, this is the HTTP body. | [optional] 
**_type** | **String** | The type of event. This SHOULD be namespaced similar to Java package naming conventions e.g. `com.example.subdomain.event.type` | [optional] 
**event_id** | **String** | The globally unique event identifier. | [optional] 
**sender** | **String** | Contains the fully-qualified ID of the user who sent this event. | [optional] 
**origin_server_ts** | **i32** | Timestamp in milliseconds on originating homeserver when this event was sent. | [optional] 
**state_key** | **String** | A unique key which defines the overwriting semantics for this piece of room state. This value is often a zero-length string. The presence of this key makes this event a State Event. State keys starting with an @ are reserved for referencing user IDs, such as room members. With the exception of a few events, state events set with a given user's ID as the state key MUST only be set by that user. | [optional] 
**unsigned** | [***::models::UnsignedData**](UnsignedData.md) |  | [optional] 
**prev_content** | [***Value**](.md) | Optional. The previous content for this event. If there is no previous content, this key will be missing. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


