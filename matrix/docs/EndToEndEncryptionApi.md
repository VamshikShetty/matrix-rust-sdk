# \EndToEndEncryptionApi

All URIs are relative to *https://matrix.org/_matrix/client/r0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**claim_keys**](EndToEndEncryptionApi.md#claim_keys) | **post** /keys/claim | Claim one-time encryption keys.
[**get_keys_changes**](EndToEndEncryptionApi.md#get_keys_changes) | **get** /keys/changes | Query users with recent device key updates.
[**query_keys**](EndToEndEncryptionApi.md#query_keys) | **post** /keys/query | Download device identity keys.
[**upload_keys**](EndToEndEncryptionApi.md#upload_keys) | **post** /keys/upload | Upload end-to-end encryption keys.



## claim_keys

> ::models::Model200ClaimKeys claim_keys(ctx, optional)
Claim one-time encryption keys.

Claims one-time keys for use in pre-key messages.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **claim_keys_request_body** | [**ClaimKeysRequestBody**](ClaimKeysRequestBody.md)| Query defining the keys to be claimed | 

### Return type

[**::models::Model200ClaimKeys**](200_claimKeys.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_keys_changes

> ::models::Model200GetKeysChanges get_keys_changes(ctx, from, to)
Query users with recent device key updates.

Gets a list of users who have updated their device identity keys since a\\nprevious sync token.\\n\\nThe server should include in the results any users who:\\n\\n* currently share a room with the calling user (ie, both users have\\n  membership state ``join``); *and*\\n* added new device identity keys or removed an existing device with\\n  identity keys, between ``from`` and ``to``.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **from** | **String**| The desired start point of the list. Should be the ``next_batch`` field\\nfrom a response to an earlier call to |/sync|. Users who have not\\nuploaded new device identity keys since this point, nor deleted\\nexisting devices with identity keys since then, will be excluded\\nfrom the results. | 
  **to** | **String**| The desired end point of the list. Should be the ``next_batch``\\nfield from a recent call to |/sync| - typically the most recent\\nsuch call. This may be used by the server as a hint to check its\\ncaches are up to date. | 

### Return type

[**::models::Model200GetKeysChanges**](200_getKeysChanges.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

