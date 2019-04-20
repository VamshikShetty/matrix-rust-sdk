# \MediaApi

All URIs are relative to *https://matrix.org/_matrix*

Method | HTTP request | Description
------------- | ------------- | -------------
[**upload_content**](MediaApi.md#upload_content) | **post** /media/r0/upload | Upload some content to the content repository.



## upload_content

> ::models::Model200MediaUpload upload_content(ctx, body, optional)
Upload some content to the content repository.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | **String**| The content to be uploaded. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | **String**| The content to be uploaded. | 
 **content_type** | **String**| The content type of the file being uploaded | 
 **filename** | **String**| The name of the file being uploaded | 

### Return type

[**::models::Model200MediaUpload**](200_media_upload.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

