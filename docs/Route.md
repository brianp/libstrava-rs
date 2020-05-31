# Route

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**athlete** | [***::models::SummaryAthlete**](SummaryAthlete.md) |  | [optional] [default to null]
**description** | **String** | The description of the route | [optional] [default to null]
**distance** | **f32** | The route&#39;s distance, in meters | [optional] [default to null]
**elevation_gain** | **f32** | The route&#39;s elevation gain. | [optional] [default to null]
**id** | **i32** | The unique identifier of this route | [optional] [default to null]
**map** | [***::models::PolylineMap**](PolylineMap.md) |  | [optional] [default to null]
**name** | **String** | The name of this route | [optional] [default to null]
**private** | **bool** | Whether this route is private | [optional] [default to null]
**starred** | **bool** | Whether this route is starred by the logged-in athlete | [optional] [default to null]
**timestamp** | **i32** | An epoch timestamp of when the route was created | [optional] [default to null]
**_type** | **i32** | This route&#39;s type (1 for ride, 2 for runs) | [optional] [default to null]
**sub_type** | **i32** | This route&#39;s sub-type (1 for road, 2 for mountain bike, 3 for cross, 4 for trail, 5 for mixed) | [optional] [default to null]
**segments** | [**Vec<::models::SummarySegment>**](SummarySegment.md) | The segments traversed by this route | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


