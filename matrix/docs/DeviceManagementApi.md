# \DeviceManagementApi

All URIs are relative to *https://matrix.org/_matrix*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_devices**](DeviceManagementApi.md#delete_devices) | **post** /client/r0/delete_devices | Bulk deletion of devices



## delete_devices

> Value delete_devices(ctx, optional)
Bulk deletion of devices

This API endpoint uses the `User-Interactive Authentication API`_. Deletes the given devices, and invalidates any access token associated with them.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **delete_devices_request_body** | [**DeleteDevicesRequestBody**](DeleteDevicesRequestBody.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

