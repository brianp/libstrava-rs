# \SegmentEffortsApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_efforts_by_segment_id**](SegmentEffortsApi.md#get_efforts_by_segment_id) | **Get** /segment_efforts | List Segment Efforts
[**get_segment_effort_by_id**](SegmentEffortsApi.md#get_segment_effort_by_id) | **Get** /segment_efforts/{id} | Get Segment Effort


# **get_efforts_by_segment_id**
> Vec<::models::DetailedSegmentEffort> get_efforts_by_segment_id(ctx, segment_id, optional)
List Segment Efforts

Returns a set of the authenticated athlete's segment efforts for a given segment.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **segment_id** | **i32**| The identifier of the segment. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **segment_id** | **i32**| The identifier of the segment. | 
 **start_date_local** | **String**| ISO 8601 formatted date time. | 
 **end_date_local** | **String**| ISO 8601 formatted date time. | 
 **per_page** | **i32**| Number of items per page. Defaults to 30. | [default to 30]

### Return type

[**Vec<::models::DetailedSegmentEffort>**](DetailedSegmentEffort.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_segment_effort_by_id**
> ::models::DetailedSegmentEffort get_segment_effort_by_id(ctx, id)
Get Segment Effort

Returns a segment effort from an activity that is owned by the authenticated athlete. Requires subscription.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| The identifier of the segment effort. | 

### Return type

[**::models::DetailedSegmentEffort**](DetailedSegmentEffort.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

