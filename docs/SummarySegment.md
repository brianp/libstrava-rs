# SummarySegment

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The unique identifier of this segment | [optional] [default to null]
**name** | **String** | The name of this segment | [optional] [default to null]
**activity_type** | **String** |  | [optional] [default to null]
**distance** | **f32** | The segment&#39;s distance, in meters | [optional] [default to null]
**average_grade** | **f32** | The segment&#39;s average grade, in percents | [optional] [default to null]
**maximum_grade** | **f32** | The segments&#39;s maximum grade, in percents | [optional] [default to null]
**elevation_high** | **f32** | The segments&#39;s highest elevation, in meters | [optional] [default to null]
**elevation_low** | **f32** | The segments&#39;s lowest elevation, in meters | [optional] [default to null]
**start_latlng** | [***::models::LatLng**](LatLng.md) |  | [optional] [default to null]
**end_latlng** | [***::models::LatLng**](LatLng.md) |  | [optional] [default to null]
**climb_category** | **i32** | The category of the climb [0, 5]. Higher is harder ie. 5 is Hors catégorie, 0 is uncategorized in climb_category. | [optional] [default to null]
**city** | **String** | The segments&#39;s city. | [optional] [default to null]
**state** | **String** | The segments&#39;s state or geographical region. | [optional] [default to null]
**country** | **String** | The segment&#39;s country. | [optional] [default to null]
**private** | **bool** | Whether this segment is private. | [optional] [default to null]
**athlete_pr_effort** | [***::models::SummarySegmentEffort**](SummarySegmentEffort.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


