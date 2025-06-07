# InlineObject4

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the file. | 
**folder_name** | Option<**String**> | The name of the project containing the file. | [optional]
**last_touched_at** | **String** | The UTC ISO 8601 time at which the file content was last modified. | 
**creator** | [**models::User**](User.md) | The user who created the file. | 
**last_touched_by** | Option<[**models::User**](User.md)> | The user who last modified the file contents. | [optional]
**thumbnail_url** | Option<**String**> | A URL to a thumbnail image of the file. | [optional]
**editor_type** | **String** | The type of editor associated with this file. | 
**role** | Option<**String**> | The role of the user making the API request in relation to the file. | [optional]
**link_access** | Option<**String**> | Access policy for users who have the link to the file. | [optional]
**url** | Option<**String**> | The URL of the file. | [optional]
**version** | Option<**String**> | The version number of the file. This number is incremented when a file is modified and can be used to check if the file has changed between requests. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


