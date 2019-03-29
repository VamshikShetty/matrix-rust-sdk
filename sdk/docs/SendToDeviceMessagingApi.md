# \SendToDeviceMessagingApi

All URIs are relative to *https://matrix.org/_matrix/client/r0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**send_to_device**](SendToDeviceMessagingApi.md#send_to_device) | **Put** /sendToDevice/{eventType}/{txnId} | Send an event to a given set of devices.



## send_to_device

> Value send_to_device(ctx, event_type, txn_id, body)
Send an event to a given set of devices.

This endpoint is used to send send-to-device events to a set of client devices.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_type** | **String**| The type of event. | 
  **txn_id** | **String**| The transaction ID for this event. Clients should generate an ID unique across requests with the same access token; it will be used by the server to ensure idempotency of requests. | 
  **body** | [**Body**](Body.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

