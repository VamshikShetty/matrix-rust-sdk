# TimelineEvents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | [***Value**](.md) | Fields in this object will vary depending on the type of event. When interacting with the REST API, this is the HTTP body. | [optional] 
**_type** | **String** | The type of event. This SHOULD be namespaced similar to Java package naming conventions e.g. `com.example.subdomain.event.type` | [optional] 
**event_id** | **String** | The globally unique event identifier. | [optional] 
**sender** | **String** | Contains the fully-qualified ID of the user who sent this event. | [optional] 
**origin_server_ts** | **i32** | Timestamp in milliseconds on originating homeserver when this event was sent. | [optional] 
**unsigned** | [***::models::UnsignedData**](UnsignedData.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


