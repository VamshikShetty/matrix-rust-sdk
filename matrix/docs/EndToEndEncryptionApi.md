# \EndToEndEncryptionApi

All URIs are relative to *http://matrix.org/_matrix/client/r0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**query_keys**](EndToEndEncryptionApi.md#query_keys) | **post** /keys/query | Download device identity keys.
[**upload_keys**](EndToEndEncryptionApi.md#upload_keys) | **post** /keys/upload | Upload end-to-end encryption keys.



## query_keys

> ::models::Model200QueryKeys query_keys(ctx, optional)
Download device identity keys.

Returns the current devices and identity keys for the given users.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query_keys** | [**QueryKeys**](QueryKeys.md)| Query defining the keys to be downloaded | 

### Return type

[**::models::Model200QueryKeys**](200_query_keys.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_keys

> ::models::Model200KeysUpload upload_keys(ctx, optional)
Upload end-to-end encryption keys.

Publishes end-to-end encryption keys for the device.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **keys_upload** | [**KeysUpload**](KeysUpload.md)| The keys to be published | 

### Return type

[**::models::Model200KeysUpload**](200_keys_upload.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

