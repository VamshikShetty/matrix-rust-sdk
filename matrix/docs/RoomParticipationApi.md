# \RoomParticipationApi

All URIs are relative to *https://matrix.org/_matrix*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_room_events**](RoomParticipationApi.md#get_room_events) | **get** /client/r0/rooms/{roomId}/messages | Get a list of events for this room
[**get_room_state_by_type**](RoomParticipationApi.md#get_room_state_by_type) | **get** /client/r0/rooms/{roomId}/state/{eventType} | Get the state identified by the type, with the empty state key.
[**get_room_state_with_key**](RoomParticipationApi.md#get_room_state_with_key) | **get** /client/r0/rooms/{roomId}/state/{eventType}/{stateKey} | Get the state identified by the type and key.
[**redact_event**](RoomParticipationApi.md#redact_event) | **put** /client/r0/rooms/{roomId}/redact/{eventId}/{txnId} | Strips all non-integrity-critical information out of an event.
[**send_event_txnid**](RoomParticipationApi.md#send_event_txnid) | **put** /client/r0/rooms/{roomId}/send/{eventType}/{txnId} | Send a message event to the given room.
[**set_room_state**](RoomParticipationApi.md#set_room_state) | **put** /client/r0/rooms/{roomId}/state/{eventType} | Send a state event to the given room.
[**set_room_state_with_key**](RoomParticipationApi.md#set_room_state_with_key) | **put** /client/r0/rooms/{roomId}/state/{eventType}/{stateKey} | Send a state event to the given room.
[**sync**](RoomParticipationApi.md#sync) | **get** /client/r0/sync | Synchronise the client's state and receive new messages.



## get_room_events

> ::models::RoomidMessages get_room_events(ctx, room_id, from, dir, optional)
Get a list of events for this room

This API returns a list of message and state events for a room. It uses pagination query parameters to paginate history in the room.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room to get events from. | 
  **from** | **String**| The token to start returning events from. This token can be obtained from a ``prev_batch`` token returned for each room by the sync API, or from a ``start`` or ``end`` token returned by a previous request to this endpoint. | 
  **dir** | **String**| The direction to return events from. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **room_id** | **String**| The room to get events from. | 
 **from** | **String**| The token to start returning events from. This token can be obtained from a ``prev_batch`` token returned for each room by the sync API, or from a ``start`` or ``end`` token returned by a previous request to this endpoint. | 
 **dir** | **String**| The direction to return events from. | 
 **to** | **String**| The token to stop returning events at. This token can be obtained from a ``prev_batch`` token returned for each room by the sync endpoint, or from a ``start`` or ``end`` token returned by a previous request to this endpoint. | 
 **limit** | **i32**| The maximum number of events to return. Default: 10. | 
 **filter** | **String**| A JSON RoomEventFilter to filter returned events with. | 

### Return type

[**::models::RoomidMessages**](roomid_messages.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_room_state_by_type

> Value get_room_state_by_type(ctx, room_id, event_type)
Get the state identified by the type, with the empty state key.

Looks up the contents of a state event in a room. If the user is joined to the room then the state is taken from the current state of the room. If the user has left the room then the state is taken from the state of the room when they left.  This looks up the state event with the empty state key.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room to look up the state in. | 
  **event_type** | **String**| The type of state to look up. | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_room_state_with_key

> Value get_room_state_with_key(ctx, room_id, event_type, state_key)
Get the state identified by the type and key.

Looks up the contents of a state event in a room. If the user is joined to the room then the state is taken from the current state of the room. If the user has left the room then the state is taken from the state of the room when they left.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room to look up the state in. | 
  **event_type** | **String**| The type of state to look up. | 
  **state_key** | **String**| The key of the state to look up. | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redact_event

> ::models::EventId redact_event(ctx, room_id, event_id, txn_id, optional)
Strips all non-integrity-critical information out of an event.

Strips all information out of an event which isn't critical to the integrity of the server-side representation of the room.  This cannot be undone.  Users may redact their own events, and any user with a power level greater than or equal to the `redact` power level of the room may redact events there.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room from which to redact the event. | 
  **event_id** | **String**| The ID of the event to redact | 
  **txn_id** | **String**| The transaction ID for this event. Clients should generate a unique ID; it will be used by the server to ensure idempotency of requests. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **room_id** | **String**| The room from which to redact the event. | 
 **event_id** | **String**| The ID of the event to redact | 
 **txn_id** | **String**| The transaction ID for this event. Clients should generate a unique ID; it will be used by the server to ensure idempotency of requests. | 
 **redact_even_id_txn_id** | [**RedactEvenIdTxnId**](RedactEvenIdTxnId.md)|  | 

### Return type

[**::models::EventId**](event_id.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_event_txnid

> ::models::EventId send_event_txnid(ctx, room_id, event_type, txn_id, optional)
Send a message event to the given room.

This endpoint is used to send a message event to a room. Message events allow access to historical events and pagination, making them suited for \"once-off\" activity in a room. The body of the request should be the content object of the event; the fields in this object will vary depending on the type of event. See `Room Events`_ for the m. event specification.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room to look up the state in. | 
  **event_type** | **String**| The type of event. | 
  **txn_id** | **String**| The transaction ID for this event. Clients should generate an ID unique across requests with the same access token; it will be used by the server to ensure idempotency of requests. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **room_id** | **String**| The room to look up the state in. | 
 **event_type** | **String**| The type of event. | 
 **txn_id** | **String**| The transaction ID for this event. Clients should generate an ID unique across requests with the same access token; it will be used by the server to ensure idempotency of requests. | 
 **body** | **Value**|  | 

### Return type

[**::models::EventId**](event_id.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_room_state

> ::models::Model200StateEventType set_room_state(ctx, room_id, event_type, optional)
Send a state event to the given room.

State events can be sent using this endpoint. This endpoint is equivalent to calling `/rooms/{roomId}/state/{eventType}/{stateKey}` with an empty `stateKey`. Previous state events with matching `<roomId>` and `<eventType>`, and empty `<stateKey>`, will be overwritten.  Requests to this endpoint **cannot use transaction IDs** like other ``PUT`` paths because they cannot be differentiated from the ``state_key``. Furthermore, ``POST`` is unsupported on state paths.  The body of the request should be the content object of the event; the fields in this object will vary depending on the type of event. See `Room Events`_ for the ``m.`` event specification. 

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room to set the state in | 
  **event_type** | **String**| The type of event to send. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **room_id** | **String**| The room to set the state in | 
 **event_type** | **String**| The type of event to send. | 
 **redact_even_id_txn_id** | [**RedactEvenIdTxnId**](RedactEvenIdTxnId.md)|  | 

### Return type

[**::models::Model200StateEventType**](200_state_eventType.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_room_state_with_key

> ::models::EventId set_room_state_with_key(ctx, room_id, event_type, state_key, optional)
Send a state event to the given room.

State events can be sent using this endpoint.  These events will be overwritten if ``<room id>``, ``<event type>`` and ``<state key>`` all match.  Requests to this endpoint **cannot use transaction IDs** like other ``PUT`` paths because they cannot be differentiated from the ``state_key``. Furthermore, ``POST`` is unsupported on state paths.  The body of the request should be the content object of the event; the fields in this object will vary depending on the type of event. See `Room Events`_ for the ``m.`` event specification. 

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room to set the state in | 
  **event_type** | **String**| The type of event to send. | 
  **state_key** | **String**| The state_key for the state to send. Defaults to the empty string. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **room_id** | **String**| The room to set the state in | 
 **event_type** | **String**| The type of event to send. | 
 **state_key** | **String**| The state_key for the state to send. Defaults to the empty string. | 
 **set_room_state_with_key_req** | [**SetRoomStateWithKeyReq**](SetRoomStateWithKeyReq.md)|  | 

### Return type

[**::models::EventId**](event_id.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync

> ::models::SyncResponse sync(ctx, optional)
Synchronise the client's state and receive new messages.

Synchronise the client's state with the latest state on the server. Clients use this API when they first log in to get an initial snapshot of the state on the server, and then continue to call this API to get incremental deltas to the state, and to receive new messages.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **filter** | **String**| The ID of a filter created using the filter API or a filter JSON object encoded as a string. The server will detect whether it is an ID or a JSON object by whether the first character is a ``\"{\"`` open brace. Passing the JSON inline is best suited to one off requests. Creating a filter using the filter API is recommended for clients that reuse the same filter multiple times, for example in long poll requests. | 
 **since** | **String**| A point in time to continue a sync from. | 
 **full_state** | **bool**| Controls whether to include the full state for all rooms the user is a member of.  If this is set to `true`, then all state events will be returned, even if `since` is non-empty. The timeline will still be limited by the `since` parameter. In this case, the ``timeout`` parameter will be ignored and the query will return immediately, possibly with an empty timeline.  If `false`, and `since` is non-empty, only state which has changed since the point indicated by `since` will be returned.  By default, this is `false`. | [default to false]
 **set_presence** | **String**| Controls whether the client is automatically marked as online by polling this API. If this parameter is omitted then the client is automatically marked as online when it uses this API. Otherwise if the parameter is set to \"offline\" then the client is not marked as being online when it uses this API. When set to \"unavailable\", the client is marked as being idle. | 
 **timeout** | **i32**| The maximum time to wait, in milliseconds, before returning this request. If no events (or other data) become available before this time elapses, the server will return a response with empty fields.  By default, this is ``0``, so the server will return immediately even if the response is empty. | [default to 0]

### Return type

[**::models::SyncResponse**](sync_response.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

