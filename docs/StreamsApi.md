# \StreamsApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_activity_streams**](StreamsApi.md#get_activity_streams) | **Get** /activities/{id}/streams | Get Activity Streams
[**get_route_streams**](StreamsApi.md#get_route_streams) | **Get** /routes/{id}/streams | Get Route Streams
[**get_segment_effort_streams**](StreamsApi.md#get_segment_effort_streams) | **Get** /segment_efforts/{id}/streams | Get Segment Effort Streams
[**get_segment_streams**](StreamsApi.md#get_segment_streams) | **Get** /segments/{id}/streams | Get Segment Streams


# **get_activity_streams**
> ::models::StreamSet get_activity_streams(ctx, id, keys, key_by_type)
Get Activity Streams

Returns the given activity's streams. Requires activity:read scope. Requires activity:read_all scope for Only Me activities.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| The identifier of the activity. | 
  **keys** | [**Vec&lt;String&gt;**](String.md)| Desired stream types. | 
  **key_by_type** | **bool**| Must be true. | [default to true]

### Return type

[**::models::StreamSet**](StreamSet.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_route_streams**
> ::models::StreamSet get_route_streams(ctx, id)
Get Route Streams

Returns the given route's streams. Requires read_all scope for private routes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| The identifier of the route. | 

### Return type

[**::models::StreamSet**](StreamSet.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_segment_effort_streams**
> ::models::StreamSet get_segment_effort_streams(ctx, id, keys, key_by_type)
Get Segment Effort Streams

Returns a set of streams for a segment effort completed by the authenticated athlete. Requires read_all scope.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| The identifier of the segment effort. | 
  **keys** | [**Vec&lt;String&gt;**](String.md)| The types of streams to return. | 
  **key_by_type** | **bool**| Must be true. | [default to true]

### Return type

[**::models::StreamSet**](StreamSet.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_segment_streams**
> ::models::StreamSet get_segment_streams(ctx, id, keys, key_by_type)
Get Segment Streams

Returns the given segment's streams. Requires read_all scope for private segments.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i64**| The identifier of the segment. | 
  **keys** | [**Vec&lt;String&gt;**](String.md)| The types of streams to return. | 
  **key_by_type** | **bool**| Must be true. | [default to true]

### Return type

[**::models::StreamSet**](StreamSet.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

