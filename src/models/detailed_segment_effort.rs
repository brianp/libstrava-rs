/* 
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * OpenAPI spec version: 3.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedSegmentEffort {
  /// The unique identifier of this effort
  #[serde(rename = "id")]
  id: Option<i64>,
  /// The effort's elapsed time
  #[serde(rename = "elapsed_time")]
  elapsed_time: Option<i32>,
  /// The time at which the effort was started.
  #[serde(rename = "start_date")]
  start_date: Option<String>,
  /// The time at which the effort was started in the local timezone.
  #[serde(rename = "start_date_local")]
  start_date_local: Option<String>,
  /// The effort's distance in meters
  #[serde(rename = "distance")]
  distance: Option<f32>,
  /// Whether this effort is the current best on the leaderboard
  #[serde(rename = "is_kom")]
  is_kom: Option<bool>,
  /// The name of the segment on which this effort was performed
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "activity")]
  activity: Option<::models::MetaActivity>,
  #[serde(rename = "athlete")]
  athlete: Option<::models::MetaAthlete>,
  /// The effort's moving time
  #[serde(rename = "moving_time")]
  moving_time: Option<i32>,
  /// The start index of this effort in its activity's stream
  #[serde(rename = "start_index")]
  start_index: Option<i32>,
  /// The end index of this effort in its activity's stream
  #[serde(rename = "end_index")]
  end_index: Option<i32>,
  /// The effort's average cadence
  #[serde(rename = "average_cadence")]
  average_cadence: Option<f32>,
  /// The average wattage of this effort
  #[serde(rename = "average_watts")]
  average_watts: Option<f32>,
  /// For riding efforts, whether the wattage was reported by a dedicated recording device
  #[serde(rename = "device_watts")]
  device_watts: Option<bool>,
  /// The heart heart rate of the athlete during this effort
  #[serde(rename = "average_heartrate")]
  average_heartrate: Option<f32>,
  /// The maximum heart rate of the athlete during this effort
  #[serde(rename = "max_heartrate")]
  max_heartrate: Option<f32>,
  #[serde(rename = "segment")]
  segment: Option<::models::SummarySegment>,
  /// The rank of the effort on the global leaderboard if it belongs in the top 10 at the time of upload
  #[serde(rename = "kom_rank")]
  kom_rank: Option<i32>,
  /// The rank of the effort on the athlete's leaderboard if it belongs in the top 3 at the time of upload
  #[serde(rename = "pr_rank")]
  pr_rank: Option<i32>,
  /// Whether this effort should be hidden when viewed within an activity
  #[serde(rename = "hidden")]
  hidden: Option<bool>
}

impl DetailedSegmentEffort {
  pub fn new() -> DetailedSegmentEffort {
    DetailedSegmentEffort {
      id: None,
      elapsed_time: None,
      start_date: None,
      start_date_local: None,
      distance: None,
      is_kom: None,
      name: None,
      activity: None,
      athlete: None,
      moving_time: None,
      start_index: None,
      end_index: None,
      average_cadence: None,
      average_watts: None,
      device_watts: None,
      average_heartrate: None,
      max_heartrate: None,
      segment: None,
      kom_rank: None,
      pr_rank: None,
      hidden: None
    }
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> DetailedSegmentEffort {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_elapsed_time(&mut self, elapsed_time: i32) {
    self.elapsed_time = Some(elapsed_time);
  }

  pub fn with_elapsed_time(mut self, elapsed_time: i32) -> DetailedSegmentEffort {
    self.elapsed_time = Some(elapsed_time);
    self
  }

  pub fn elapsed_time(&self) -> Option<&i32> {
    self.elapsed_time.as_ref()
  }

  pub fn reset_elapsed_time(&mut self) {
    self.elapsed_time = None;
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> DetailedSegmentEffort {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_start_date_local(&mut self, start_date_local: String) {
    self.start_date_local = Some(start_date_local);
  }

  pub fn with_start_date_local(mut self, start_date_local: String) -> DetailedSegmentEffort {
    self.start_date_local = Some(start_date_local);
    self
  }

  pub fn start_date_local(&self) -> Option<&String> {
    self.start_date_local.as_ref()
  }

  pub fn reset_start_date_local(&mut self) {
    self.start_date_local = None;
  }

  pub fn set_distance(&mut self, distance: f32) {
    self.distance = Some(distance);
  }

  pub fn with_distance(mut self, distance: f32) -> DetailedSegmentEffort {
    self.distance = Some(distance);
    self
  }

  pub fn distance(&self) -> Option<&f32> {
    self.distance.as_ref()
  }

  pub fn reset_distance(&mut self) {
    self.distance = None;
  }

  pub fn set_is_kom(&mut self, is_kom: bool) {
    self.is_kom = Some(is_kom);
  }

  pub fn with_is_kom(mut self, is_kom: bool) -> DetailedSegmentEffort {
    self.is_kom = Some(is_kom);
    self
  }

  pub fn is_kom(&self) -> Option<&bool> {
    self.is_kom.as_ref()
  }

  pub fn reset_is_kom(&mut self) {
    self.is_kom = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> DetailedSegmentEffort {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_activity(&mut self, activity: ::models::MetaActivity) {
    self.activity = Some(activity);
  }

  pub fn with_activity(mut self, activity: ::models::MetaActivity) -> DetailedSegmentEffort {
    self.activity = Some(activity);
    self
  }

  pub fn activity(&self) -> Option<&::models::MetaActivity> {
    self.activity.as_ref()
  }

  pub fn reset_activity(&mut self) {
    self.activity = None;
  }

  pub fn set_athlete(&mut self, athlete: ::models::MetaAthlete) {
    self.athlete = Some(athlete);
  }

  pub fn with_athlete(mut self, athlete: ::models::MetaAthlete) -> DetailedSegmentEffort {
    self.athlete = Some(athlete);
    self
  }

  pub fn athlete(&self) -> Option<&::models::MetaAthlete> {
    self.athlete.as_ref()
  }

  pub fn reset_athlete(&mut self) {
    self.athlete = None;
  }

  pub fn set_moving_time(&mut self, moving_time: i32) {
    self.moving_time = Some(moving_time);
  }

  pub fn with_moving_time(mut self, moving_time: i32) -> DetailedSegmentEffort {
    self.moving_time = Some(moving_time);
    self
  }

  pub fn moving_time(&self) -> Option<&i32> {
    self.moving_time.as_ref()
  }

  pub fn reset_moving_time(&mut self) {
    self.moving_time = None;
  }

  pub fn set_start_index(&mut self, start_index: i32) {
    self.start_index = Some(start_index);
  }

  pub fn with_start_index(mut self, start_index: i32) -> DetailedSegmentEffort {
    self.start_index = Some(start_index);
    self
  }

  pub fn start_index(&self) -> Option<&i32> {
    self.start_index.as_ref()
  }

  pub fn reset_start_index(&mut self) {
    self.start_index = None;
  }

  pub fn set_end_index(&mut self, end_index: i32) {
    self.end_index = Some(end_index);
  }

  pub fn with_end_index(mut self, end_index: i32) -> DetailedSegmentEffort {
    self.end_index = Some(end_index);
    self
  }

  pub fn end_index(&self) -> Option<&i32> {
    self.end_index.as_ref()
  }

  pub fn reset_end_index(&mut self) {
    self.end_index = None;
  }

  pub fn set_average_cadence(&mut self, average_cadence: f32) {
    self.average_cadence = Some(average_cadence);
  }

  pub fn with_average_cadence(mut self, average_cadence: f32) -> DetailedSegmentEffort {
    self.average_cadence = Some(average_cadence);
    self
  }

  pub fn average_cadence(&self) -> Option<&f32> {
    self.average_cadence.as_ref()
  }

  pub fn reset_average_cadence(&mut self) {
    self.average_cadence = None;
  }

  pub fn set_average_watts(&mut self, average_watts: f32) {
    self.average_watts = Some(average_watts);
  }

  pub fn with_average_watts(mut self, average_watts: f32) -> DetailedSegmentEffort {
    self.average_watts = Some(average_watts);
    self
  }

  pub fn average_watts(&self) -> Option<&f32> {
    self.average_watts.as_ref()
  }

  pub fn reset_average_watts(&mut self) {
    self.average_watts = None;
  }

  pub fn set_device_watts(&mut self, device_watts: bool) {
    self.device_watts = Some(device_watts);
  }

  pub fn with_device_watts(mut self, device_watts: bool) -> DetailedSegmentEffort {
    self.device_watts = Some(device_watts);
    self
  }

  pub fn device_watts(&self) -> Option<&bool> {
    self.device_watts.as_ref()
  }

  pub fn reset_device_watts(&mut self) {
    self.device_watts = None;
  }

  pub fn set_average_heartrate(&mut self, average_heartrate: f32) {
    self.average_heartrate = Some(average_heartrate);
  }

  pub fn with_average_heartrate(mut self, average_heartrate: f32) -> DetailedSegmentEffort {
    self.average_heartrate = Some(average_heartrate);
    self
  }

  pub fn average_heartrate(&self) -> Option<&f32> {
    self.average_heartrate.as_ref()
  }

  pub fn reset_average_heartrate(&mut self) {
    self.average_heartrate = None;
  }

  pub fn set_max_heartrate(&mut self, max_heartrate: f32) {
    self.max_heartrate = Some(max_heartrate);
  }

  pub fn with_max_heartrate(mut self, max_heartrate: f32) -> DetailedSegmentEffort {
    self.max_heartrate = Some(max_heartrate);
    self
  }

  pub fn max_heartrate(&self) -> Option<&f32> {
    self.max_heartrate.as_ref()
  }

  pub fn reset_max_heartrate(&mut self) {
    self.max_heartrate = None;
  }

  pub fn set_segment(&mut self, segment: ::models::SummarySegment) {
    self.segment = Some(segment);
  }

  pub fn with_segment(mut self, segment: ::models::SummarySegment) -> DetailedSegmentEffort {
    self.segment = Some(segment);
    self
  }

  pub fn segment(&self) -> Option<&::models::SummarySegment> {
    self.segment.as_ref()
  }

  pub fn reset_segment(&mut self) {
    self.segment = None;
  }

  pub fn set_kom_rank(&mut self, kom_rank: i32) {
    self.kom_rank = Some(kom_rank);
  }

  pub fn with_kom_rank(mut self, kom_rank: i32) -> DetailedSegmentEffort {
    self.kom_rank = Some(kom_rank);
    self
  }

  pub fn kom_rank(&self) -> Option<&i32> {
    self.kom_rank.as_ref()
  }

  pub fn reset_kom_rank(&mut self) {
    self.kom_rank = None;
  }

  pub fn set_pr_rank(&mut self, pr_rank: i32) {
    self.pr_rank = Some(pr_rank);
  }

  pub fn with_pr_rank(mut self, pr_rank: i32) -> DetailedSegmentEffort {
    self.pr_rank = Some(pr_rank);
    self
  }

  pub fn pr_rank(&self) -> Option<&i32> {
    self.pr_rank.as_ref()
  }

  pub fn reset_pr_rank(&mut self) {
    self.pr_rank = None;
  }

  pub fn set_hidden(&mut self, hidden: bool) {
    self.hidden = Some(hidden);
  }

  pub fn with_hidden(mut self, hidden: bool) -> DetailedSegmentEffort {
    self.hidden = Some(hidden);
    self
  }

  pub fn hidden(&self) -> Option<&bool> {
    self.hidden.as_ref()
  }

  pub fn reset_hidden(&mut self) {
    self.hidden = None;
  }

}


