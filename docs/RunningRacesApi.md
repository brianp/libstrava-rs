# \RunningRacesApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_running_race_by_id**](RunningRacesApi.md#get_running_race_by_id) | **Get** /running_races/{id} | Get Running Race
[**get_running_races**](RunningRacesApi.md#get_running_races) | **Get** /running_races | List Running Races


# **get_running_race_by_id**
> ::models::RunningRace get_running_race_by_id(ctx, id)
Get Running Race

Returns a running race for a given identifier.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the running race. | 

### Return type

[**::models::RunningRace**](RunningRace.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_running_races**
> Vec<::models::RunningRace> get_running_races(ctx, optional)
List Running Races

Returns a list running races based on a set of search criteria.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **year** | **i32**| Filters the list by a given year. | 

### Return type

[**Vec<::models::RunningRace>**](RunningRace.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

