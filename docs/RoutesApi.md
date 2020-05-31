# \RoutesApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_route_as_gpx**](RoutesApi.md#get_route_as_gpx) | **Get** /routes/{id}/export_gpx | Export Route GPX
[**get_route_as_tcx**](RoutesApi.md#get_route_as_tcx) | **Get** /routes/{id}/export_tcx | Export Route TCX
[**get_route_by_id**](RoutesApi.md#get_route_by_id) | **Get** /routes/{id} | Get Route
[**get_routes_by_athlete_id**](RoutesApi.md#get_routes_by_athlete_id) | **Get** /athletes/{id}/routes | List Athlete Routes


# **get_route_as_gpx**
> get_route_as_gpx(ctx, id)
Export Route GPX

Returns a GPX file of the route. Requires read_all scope for private routes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the route. | 

### Return type

 (empty response body)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_route_as_tcx**
> get_route_as_tcx(ctx, id)
Export Route TCX

Returns a TCX file of the route. Requires read_all scope for private routes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the route. | 

### Return type

 (empty response body)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_route_by_id**
> ::models::Route get_route_by_id(ctx, id)
Get Route

Returns a route using its identifier. Requires read_all scope for private routes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the route. | 

### Return type

[**::models::Route**](Route.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_routes_by_athlete_id**
> Vec<::models::Route> get_routes_by_athlete_id(ctx, id, optional)
List Athlete Routes

Returns a list of the routes created by the authenticated athlete using their athlete ID. Private routes are filtered out unless requested by a token with read_all scope.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the athlete. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i32**| The identifier of the athlete. | 
 **page** | **i32**| Page number. Defaults to 1. | 
 **per_page** | **i32**| Number of items per page. Defaults to 30. | [default to 30]

### Return type

[**Vec<::models::Route>**](Route.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

