# Model200Register

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | **String** | An access token for the account. This access token can then be used to authorize other requests. Required if the ``inhibit_login`` option is false. | [optional] 
**device_id** | **String** | ID of the registered device. Will be the same as the corresponding parameter in the request, if one was specified. Required if the ``inhibit_login`` option is false. | [optional] 
**user_id** | **String** | The fully-qualified Matrix user ID (MXID) that has been registered.  Any user ID returned by this API must conform to the grammar given in the `Matrix specification <https://matrix.org/docs/spec/appendices.html#user-identifiers>`_. | 
**home_server** | **String** | The server_name of the homeserver on which the account has been registered.  **Deprecated**. Clients should extract the server_name from ``user_id`` (by splitting at the first colon) if they require it. Note also that ``homeserver`` is not spelt this way. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


