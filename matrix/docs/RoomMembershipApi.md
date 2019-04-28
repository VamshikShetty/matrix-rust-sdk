# \RoomMembershipApi

All URIs are relative to *https://matrix.org/_matrix*

Method | HTTP request | Description
------------- | ------------- | -------------
[**forget_room**](RoomMembershipApi.md#forget_room) | **post** /client/r0/rooms/{roomId}/forget | Stop the requesting user remembering about a particular room.
[**invite_user**](RoomMembershipApi.md#invite_user) | **post** /client/r0/rooms/{roomId}/invite | Invite a user to participate in a particular room.
[**join_room**](RoomMembershipApi.md#join_room) | **post** /client/r0/join/{roomIdOrAlias} | Start the requesting user participating in a particular room.
[**leave_room**](RoomMembershipApi.md#leave_room) | **post** /client/r0/rooms/{roomId}/leave | Stop the requesting user participating in a particular room.



## forget_room

> Value forget_room(ctx, room_id)
Stop the requesting user remembering about a particular room.

This API stops a user remembering about a particular room.  In general, history is a first class citizen in Matrix. After this API is called, however, a user will no longer be able to retrieve history for this room. If all users on a homeserver forget a room, the room is eligible for deletion from that homeserver.  If the user is currently joined to the room, they must leave the room before calling this API.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room identifier to forget. | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_user

> Value invite_user(ctx, room_id, user_id)
Invite a user to participate in a particular room.

.. _invite-by-user-id-endpoint:  *Note that there are two forms of this API, which are documented separately. This version of the API requires that the inviter knows the Matrix identifier of the invitee. The other is documented in the* `third party invites section`_.  This API invites a user to participate in a particular room. They do not start participating in the room until they actually join the room.  Only users currently in a particular room can invite other users to join that room.  If the user was invited to the room, the homeserver will append a ``m.room.member`` event to the room.  .. _third party invites section: `invite-by-third-party-id-endpoint`_

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room identifier (not alias) to which to invite the user. | 
  **user_id** | [**UserId**](UserId.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## leave_room

> Value leave_room(ctx, room_id)
Stop the requesting user participating in a particular room.

This API stops a user participating in a particular room.  If the user was already in the room, they will no longer be able to see new events in the room. If the room requires an invite to join, they will need to be re-invited before they can re-join.  If the user was invited to the room, but had not joined, this call serves to reject the invite.  The user will still be allowed to retrieve history from the room which they were previously allowed to see.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_id** | **String**| The room identifier to leave. | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

