# \AthletesApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_logged_in_athlete**](AthletesApi.md#get_logged_in_athlete) | **Get** /athlete | Get Authenticated Athlete
[**get_logged_in_athlete_zones**](AthletesApi.md#get_logged_in_athlete_zones) | **Get** /athlete/zones | Get Zones
[**get_stats**](AthletesApi.md#get_stats) | **Get** /athletes/{id}/stats | Get Athlete Stats
[**update_logged_in_athlete**](AthletesApi.md#update_logged_in_athlete) | **Put** /athlete | Update Athlete


# **get_logged_in_athlete**
> ::models::DetailedAthlete get_logged_in_athlete(ctx, )
Get Authenticated Athlete

Returns the currently authenticated athlete. Tokens with profile:read_all scope will receive a detailed athlete representation; all others will receive a summary representation.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DetailedAthlete**](DetailedAthlete.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_logged_in_athlete_zones**
> ::models::Zones get_logged_in_athlete_zones(ctx, )
Get Zones

Returns the the authenticated athlete's heart rate and power zones. Requires profile:read_all.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Zones**](Zones.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_stats**
> ::models::ActivityStats get_stats(ctx, id)
Get Athlete Stats

Returns the activity stats of an athlete. Only includes data from activities set to Everyone visibilty.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the athlete. Must match the authenticated athlete. | 

### Return type

[**::models::ActivityStats**](ActivityStats.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_logged_in_athlete**
> ::models::DetailedAthlete update_logged_in_athlete(ctx, weight)
Update Athlete

Update the currently authenticated athlete. Requires profile:write scope.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **weight** | **f32**| The weight of the athlete in kilograms. | 

### Return type

[**::models::DetailedAthlete**](DetailedAthlete.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

