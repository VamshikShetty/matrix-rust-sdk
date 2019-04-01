# \RoomParticipationApi

All URIs are relative to *https://matrix.org/_matrix/client/r0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**send_event_txnid**](RoomParticipationApi.md#send_event_txnid) | **put** /rooms/{roomId}/send/{eventType}/{txnId} | Send a message event to the given room.



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

