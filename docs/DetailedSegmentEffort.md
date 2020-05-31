# DetailedSegmentEffort

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The unique identifier of this effort | [optional] [default to null]
**elapsed_time** | **i32** | The effort&#39;s elapsed time | [optional] [default to null]
**start_date** | **String** | The time at which the effort was started. | [optional] [default to null]
**start_date_local** | **String** | The time at which the effort was started in the local timezone. | [optional] [default to null]
**distance** | **f32** | The effort&#39;s distance in meters | [optional] [default to null]
**is_kom** | **bool** | Whether this effort is the current best on the leaderboard | [optional] [default to null]
**name** | **String** | The name of the segment on which this effort was performed | [optional] [default to null]
**activity** | [***::models::MetaActivity**](MetaActivity.md) |  | [optional] [default to null]
**athlete** | [***::models::MetaAthlete**](MetaAthlete.md) |  | [optional] [default to null]
**moving_time** | **i32** | The effort&#39;s moving time | [optional] [default to null]
**start_index** | **i32** | The start index of this effort in its activity&#39;s stream | [optional] [default to null]
**end_index** | **i32** | The end index of this effort in its activity&#39;s stream | [optional] [default to null]
**average_cadence** | **f32** | The effort&#39;s average cadence | [optional] [default to null]
**average_watts** | **f32** | The average wattage of this effort | [optional] [default to null]
**device_watts** | **bool** | For riding efforts, whether the wattage was reported by a dedicated recording device | [optional] [default to null]
**average_heartrate** | **f32** | The heart heart rate of the athlete during this effort | [optional] [default to null]
**max_heartrate** | **f32** | The maximum heart rate of the athlete during this effort | [optional] [default to null]
**segment** | [***::models::SummarySegment**](SummarySegment.md) |  | [optional] [default to null]
**kom_rank** | **i32** | The rank of the effort on the global leaderboard if it belongs in the top 10 at the time of upload | [optional] [default to null]
**pr_rank** | **i32** | The rank of the effort on the athlete&#39;s leaderboard if it belongs in the top 3 at the time of upload | [optional] [default to null]
**hidden** | **bool** | Whether this effort should be hidden when viewed within an activity | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


