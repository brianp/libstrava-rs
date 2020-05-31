# \ClubsApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_club_activities_by_id**](ClubsApi.md#get_club_activities_by_id) | **Get** /clubs/{id}/activities | List Club Activities
[**get_club_admins_by_id**](ClubsApi.md#get_club_admins_by_id) | **Get** /clubs/{id}/admins | List Club Administrators
[**get_club_by_id**](ClubsApi.md#get_club_by_id) | **Get** /clubs/{id} | Get Club
[**get_club_members_by_id**](ClubsApi.md#get_club_members_by_id) | **Get** /clubs/{id}/members | List Club Members
[**get_logged_in_athlete_clubs**](ClubsApi.md#get_logged_in_athlete_clubs) | **Get** /athlete/clubs | List Athlete Clubs


# **get_club_activities_by_id**
> Vec<::models::SummaryActivity> get_club_activities_by_id(ctx, id, optional)
List Club Activities

Retrieve recent activities from members of a specific club. The authenticated athlete must belong to the requested club in order to hit this endpoint. Pagination is supported. Athlete profile visibility is respected for all activities.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the club. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i32**| The identifier of the club. | 
 **page** | **i32**| Page number. Defaults to 1. | 
 **per_page** | **i32**| Number of items per page. Defaults to 30. | [default to 30]

### Return type

[**Vec<::models::SummaryActivity>**](SummaryActivity.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_club_admins_by_id**
> Vec<::models::SummaryAthlete> get_club_admins_by_id(ctx, id, optional)
List Club Administrators

Returns a list of the administrators of a given club.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the club. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i32**| The identifier of the club. | 
 **page** | **i32**| Page number. Defaults to 1. | 
 **per_page** | **i32**| Number of items per page. Defaults to 30. | [default to 30]

### Return type

[**Vec<::models::SummaryAthlete>**](SummaryAthlete.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_club_by_id**
> ::models::DetailedClub get_club_by_id(ctx, id)
Get Club

Returns a given club using its identifier.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the club. | 

### Return type

[**::models::DetailedClub**](DetailedClub.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_club_members_by_id**
> Vec<::models::SummaryAthlete> get_club_members_by_id(ctx, id, optional)
List Club Members

Returns a list of the athletes who are members of a given club.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| The identifier of the club. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **i32**| The identifier of the club. | 
 **page** | **i32**| Page number. Defaults to 1. | 
 **per_page** | **i32**| Number of items per page. Defaults to 30. | [default to 30]

### Return type

[**Vec<::models::SummaryAthlete>**](SummaryAthlete.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_logged_in_athlete_clubs**
> Vec<::models::SummaryClub> get_logged_in_athlete_clubs(ctx, optional)
List Athlete Clubs

Returns a list of the clubs whose membership includes the authenticated athlete.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **i32**| Page number. Defaults to 1. | 
 **per_page** | **i32**| Number of items per page. Defaults to 30. | [default to 30]

### Return type

[**Vec<::models::SummaryClub>**](SummaryClub.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

