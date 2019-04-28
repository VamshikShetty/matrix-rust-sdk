# \RoomMembershipApi

All URIs are relative to *https://matrix.org/_matrix*

Method | HTTP request | Description
------------- | ------------- | -------------
[**join_room**](RoomMembershipApi.md#join_room) | **post** /client/r0/join/{roomIdOrAlias} | Start the requesting user participating in a particular room.



## join_room

> ::models::RoomId join_room(ctx, room_id_or_alias, optional)
Start the requesting user participating in a particular room.

*Note that this API takes either a room ID or alias, unlike* ``/room/{roomId}/join``.  This API starts a user participating in a particular room, if that user is allowed to participate in that room. After this call, the client is allowed to see all current state events in the room, and all subsequent events associated with the room until the user leaves the room.  After a user has joined a room, the room will appear as an entry in the response of the |/initialSync|_ and |/sync|_ APIs.  If a ``third_party_signed`` was supplied, the homeserver must verify that it matches a pending ``m.room.third_party_invite`` event in the room, and perform key validity checking if required by the event.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id_or_alias** | **String**| The room identifier or alias to join. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **room_id_or_alias** | **String**| The room identifier or alias to join. | 
 **server_name** | [**Vec<String>**](String.md)| The servers to attempt to join the room through. One of the servers must be participating in the room. | 
 **join_roomid_req** | [**JoinRoomidReq**](JoinRoomidReq.md)|  | 

### Return type

[**::models::RoomId**](room_id.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

