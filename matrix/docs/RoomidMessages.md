# RoomidMessages

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start** | **String** | The token the pagination starts from. If ``dir=b`` this will be the token supplied in ``from``. | [optional] 
**chunk** | [**Vec<::models::MessagesRoomevent>**](messages_roomevent.md) | A list of room events. | [optional] 
**end** | **String** | The token the pagination ends at. If ``dir=b`` this token should be used again to request even earlier events. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


