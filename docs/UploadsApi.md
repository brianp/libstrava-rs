# \UploadsApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_upload**](UploadsApi.md#create_upload) | **Post** /uploads | Upload Activity
[**get_upload_by_id**](UploadsApi.md#get_upload_by_id) | **Get** /uploads/{uploadId} | Get Upload


# **create_upload**
> ::models::Upload create_upload(ctx, optional)
Upload Activity

Uploads a new data file to create an activity from. Requires activity:write scope.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file** | **File**| The uploaded file. | 
 **name** | **String**| The desired name of the resulting activity. | 
 **description** | **String**| The desired description of the resulting activity. | 
 **trainer** | **String**| Whether the resulting activity should be marked as having been performed on a trainer. | 
 **commute** | **String**| Whether the resulting activity should be tagged as a commute. | 
 **data_type** | **String**| The format of the uploaded file. | 
 **external_id** | **String**| The desired external identifier of the resulting activity. | 

### Return type

[**::models::Upload**](Upload.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_upload_by_id**
> ::models::Upload get_upload_by_id(ctx, upload_id)
Get Upload

Returns an upload for a given identifier. Requires activity:write scope.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **upload_id** | **i64**| The identifier of the upload. | 

### Return type

[**::models::Upload**](Upload.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

