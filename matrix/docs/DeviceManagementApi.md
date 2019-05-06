# \DeviceManagementApi

All URIs are relative to *https://matrix.org/_matrix*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_device**](DeviceManagementApi.md#delete_device) | **delete** /client/r0/devices/{deviceId} | Delete a device
[**delete_devices**](DeviceManagementApi.md#delete_devices) | **post** /client/r0/delete_devices | Bulk deletion of devices
[**get_device**](DeviceManagementApi.md#get_device) | **get** /client/r0/devices/{deviceId} | Get a single device
[**get_devices**](DeviceManagementApi.md#get_devices) | **get** /client/r0/devices | List registered devices for the current user
[**update_device**](DeviceManagementApi.md#update_device) | **put** /client/r0/devices/{deviceId} | Update a device



## delete_device

> Value delete_device(ctx, device_id, optional)
Delete a device

This API endpoint uses the `User-Interactive Authentication API`_.  Deletes the given device, and invalidates any access token associated with it.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **device_id** | **String**| The device to delete. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **device_id** | **String**| The device to delete. | 
 **device_delete_req** | [**DeviceDeleteReq**](DeviceDeleteReq.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## get_device

> ::models::Device get_device(ctx, device_id)
Get a single device

Gets information on a single device, by device id.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **device_id** | **String**| The device to retrieve. | 

### Return type

[**::models::Device**](Device.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_devices

> ::models::ListOfDevice get_devices(ctx, )
List registered devices for the current user

Gets information about all devices for the current user.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::ListOfDevice**](list_of_device.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_device

> Value update_device(ctx, device_id, display_name)
Update a device

Updates the metadata on the given device.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **device_id** | **String**| The device to update. | 
  **display_name** | [**DisplayName**](DisplayName.md)| New information for the device. | 

### Return type

[**Value**](Value.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

