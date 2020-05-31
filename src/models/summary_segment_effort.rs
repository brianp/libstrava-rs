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
pub struct SummarySegmentEffort {
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
  is_kom: Option<bool>
}

impl SummarySegmentEffort {
  pub fn new() -> SummarySegmentEffort {
    SummarySegmentEffort {
      id: None,
      elapsed_time: None,
      start_date: None,
      start_date_local: None,
      distance: None,
      is_kom: None
    }
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> SummarySegmentEffort {
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

  pub fn with_elapsed_time(mut self, elapsed_time: i32) -> SummarySegmentEffort {
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

  pub fn with_start_date(mut self, start_date: String) -> SummarySegmentEffort {
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

  pub fn with_start_date_local(mut self, start_date_local: String) -> SummarySegmentEffort {
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

  pub fn with_distance(mut self, distance: f32) -> SummarySegmentEffort {
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

  pub fn with_is_kom(mut self, is_kom: bool) -> SummarySegmentEffort {
    self.is_kom = Some(is_kom);
    self
  }

  pub fn is_kom(&self) -> Option<&bool> {
    self.is_kom.as_ref()
  }

  pub fn reset_is_kom(&mut self) {
    self.is_kom = None;
  }

}


