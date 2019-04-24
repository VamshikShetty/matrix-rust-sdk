# \RoomDirectoryApi

All URIs are relative to *https://matrix.org/_matrix*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_room_alias**](RoomDirectoryApi.md#delete_room_alias) | **delete** /client/r0/directory/room/{roomAlias} | Remove a mapping of room alias to room ID.
[**get_room_id_by_alias**](RoomDirectoryApi.md#get_room_id_by_alias) | **get** /client/r0/directory/room/{roomAlias} | Get the room ID corresponding to this room alias.
[**set_room_alias**](RoomDirectoryApi.md#set_room_alias) | **put** /client/r0/directory/room/{roomAlias} | Create a new mapping from room alias to room ID.



## delete_room_alias

> Value delete_room_alias(ctx, room_alias)
Remove a mapping of room alias to room ID.

Remove a mapping of room alias to room ID. Servers may choose to implement additional access control checks here, for instance that room aliases can only be deleted by their creator or a server administrator.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_alias** | **String**| The room alias to remove. | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_room_id_by_alias

> ::models::RoomDirRequestBody get_room_id_by_alias(room_alias)
Get the room ID corresponding to this room alias.

Requests that the server resolve a room alias to a room ID.  The server will use the federation API to resolve the alias if the domain part of the alias does not correspond to the server's own domain.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **room_alias** | **String**| The room alias. | 

### Return type

[**::models::RoomDirRequestBody**](room_dir_request_body.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_room_alias

> Value set_room_alias(ctx, room_alias, optional)
Create a new mapping from room alias to room ID.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **room_alias** | **String**| The room alias to set. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **room_alias** | **String**| The room alias to set. | 
 **room_id** | [**RoomId**](RoomId.md)| Information about this room alias. | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

