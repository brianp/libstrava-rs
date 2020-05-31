# DetailedAthlete

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The unique identifier of the athlete | [optional] [default to null]
**resource_state** | **i32** | Resource state, indicates level of detail. Possible values: 1 -&gt; \&quot;meta\&quot;, 2 -&gt; \&quot;summary\&quot;, 3 -&gt; \&quot;detail\&quot; | [optional] [default to null]
**firstname** | **String** | The athlete&#39;s first name. | [optional] [default to null]
**lastname** | **String** | The athlete&#39;s last name. | [optional] [default to null]
**profile_medium** | **String** | URL to a 62x62 pixel profile picture. | [optional] [default to null]
**profile** | **String** | URL to a 124x124 pixel profile picture. | [optional] [default to null]
**city** | **String** | The athlete&#39;s city. | [optional] [default to null]
**state** | **String** | The athlete&#39;s state or geographical region. | [optional] [default to null]
**country** | **String** | The athlete&#39;s country. | [optional] [default to null]
**sex** | **String** | The athlete&#39;s sex. | [optional] [default to null]
**premium** | **bool** | Deprecated.  Use summit field instead. Whether the athlete has any Summit subscription. | [optional] [default to null]
**summit** | **bool** | Whether the athlete has any Summit subscription. | [optional] [default to null]
**created_at** | **String** | The time at which the athlete was created. | [optional] [default to null]
**updated_at** | **String** | The time at which the athlete was last updated. | [optional] [default to null]
**follower_count** | **i32** | The athlete&#39;s follower count. | [optional] [default to null]
**friend_count** | **i32** | The athlete&#39;s friend count. | [optional] [default to null]
**measurement_preference** | **String** | The athlete&#39;s preferred unit system. | [optional] [default to null]
**ftp** | **i32** | The athlete&#39;s FTP (Functional Threshold Power). | [optional] [default to null]
**weight** | **f32** | The athlete&#39;s weight. | [optional] [default to null]
**clubs** | [**Vec<::models::SummaryClub>**](SummaryClub.md) | The athlete&#39;s clubs. | [optional] [default to null]
**bikes** | [**Vec<::models::SummaryGear>**](SummaryGear.md) | The athlete&#39;s bikes. | [optional] [default to null]
**shoes** | [**Vec<::models::SummaryGear>**](SummaryGear.md) | The athlete&#39;s shoes. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


