# \RoomParticipationApi

All URIs are relative to *https://matrix.org/_matrix*

Method | HTTP request | Description
------------- | ------------- | -------------
[**send_event_txnid**](RoomParticipationApi.md#send_event_txnid) | **put** /client/r0/rooms/{roomId}/send/{eventType}/{txnId} | Send a message event to the given room.
[**sync**](RoomParticipationApi.md#sync) | **get** /client/r0/sync | Synchronise the client's state and receive new messages.



## send_event_txnid

> ::models::Model200SendEventTxnid send_event_txnid(ctx, room_id, event_type, txn_id, optional)
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

[**::models::Model200SendEventTxnid**](200_sendEventTxnid.md)

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

