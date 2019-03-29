# \SessionManagementApi

All URIs are relative to *https://matrix.org/_matrix/client/r0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_login_flows**](SessionManagementApi.md#get_login_flows) | **Get** /login | Get the supported login types to authenticate users
[**login**](SessionManagementApi.md#login) | **Post** /login | Authenticates the user.



## get_login_flows

> ::models::Model200LoginGet get_login_flows()
Get the supported login types to authenticate users

Gets the homeserver's supported login types to authenticate users Clients should pick one of these and supply it as the ``type`` when logging in.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::Model200LoginGet**](200_login_get.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> ::models::Model200LoginPut login(login_request_body)
Authenticates the user.

Authenticates the user, and issues an access token they can use to authorize themself in subsequent requests. If the client does not supply a `device_id`, the server must auto-generate one. The returned access token must be associated with the `device_id` supplied by the client or generated by the server. The server may invalidate any access token previously associated with that device. See `Relationship between access tokens and devices`.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **login_request_body** | [**LoginRequestBody**](LoginRequestBody.md)|  | 

### Return type

[**::models::Model200LoginPut**](200_login_put.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
