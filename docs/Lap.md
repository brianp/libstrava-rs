# Lap

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The unique identifier of this lap | [optional] [default to null]
**activity** | [***::models::MetaActivity**](MetaActivity.md) |  | [optional] [default to null]
**athlete** | [***::models::MetaAthlete**](MetaAthlete.md) |  | [optional] [default to null]
**average_cadence** | **f32** | The lap&#39;s average cadence | [optional] [default to null]
**average_speed** | **f32** | The lap&#39;s average speed | [optional] [default to null]
**distance** | **f32** | The lap&#39;s distance, in meters | [optional] [default to null]
**elapsed_time** | **i32** | The lap&#39;s elapsed time, in seconds | [optional] [default to null]
**start_index** | **i32** | The start index of this effort in its activity&#39;s stream | [optional] [default to null]
**end_index** | **i32** | The end index of this effort in its activity&#39;s stream | [optional] [default to null]
**lap_index** | **i32** | The index of this lap in the activity it belongs to | [optional] [default to null]
**max_speed** | **f32** | The maximum speed of this lat, in meters per second | [optional] [default to null]
**moving_time** | **i32** | The lap&#39;s moving time, in seconds | [optional] [default to null]
**name** | **String** | The name of the lap | [optional] [default to null]
**pace_zone** | **i32** | The athlete&#39;s pace zone during this lap | [optional] [default to null]
**split** | **i32** |  | [optional] [default to null]
**start_date** | **String** | The time at which the lap was started. | [optional] [default to null]
**start_date_local** | **String** | The time at which the lap was started in the local timezone. | [optional] [default to null]
**total_elevation_gain** | **f32** | The elevation gain of this lap, in meters | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


