/* 
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * OpenAPI spec version: 3.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ActivityStats : A set of rolled-up statistics and totals for an athlete

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityStats {
  /// The longest distance ridden by the athlete.
  #[serde(rename = "biggest_ride_distance")]
  biggest_ride_distance: Option<f64>,
  /// The highest climb ridden by the athlete.
  #[serde(rename = "biggest_climb_elevation_gain")]
  biggest_climb_elevation_gain: Option<f64>,
  /// The recent (last 4 weeks) ride stats for the athlete.
  #[serde(rename = "recent_ride_totals")]
  recent_ride_totals: Option<::models::ActivityTotal>,
  /// The recent (last 4 weeks) run stats for the athlete.
  #[serde(rename = "recent_run_totals")]
  recent_run_totals: Option<::models::ActivityTotal>,
  /// The recent (last 4 weeks) swim stats for the athlete.
  #[serde(rename = "recent_swim_totals")]
  recent_swim_totals: Option<::models::ActivityTotal>,
  /// The year to date ride stats for the athlete.
  #[serde(rename = "ytd_ride_totals")]
  ytd_ride_totals: Option<::models::ActivityTotal>,
  /// The year to date run stats for the athlete.
  #[serde(rename = "ytd_run_totals")]
  ytd_run_totals: Option<::models::ActivityTotal>,
  /// The year to date swim stats for the athlete.
  #[serde(rename = "ytd_swim_totals")]
  ytd_swim_totals: Option<::models::ActivityTotal>,
  /// The all time ride stats for the athlete.
  #[serde(rename = "all_ride_totals")]
  all_ride_totals: Option<::models::ActivityTotal>,
  /// The all time run stats for the athlete.
  #[serde(rename = "all_run_totals")]
  all_run_totals: Option<::models::ActivityTotal>,
  /// The all time swim stats for the athlete.
  #[serde(rename = "all_swim_totals")]
  all_swim_totals: Option<::models::ActivityTotal>
}

impl ActivityStats {
  /// A set of rolled-up statistics and totals for an athlete
  pub fn new() -> ActivityStats {
    ActivityStats {
      biggest_ride_distance: None,
      biggest_climb_elevation_gain: None,
      recent_ride_totals: None,
      recent_run_totals: None,
      recent_swim_totals: None,
      ytd_ride_totals: None,
      ytd_run_totals: None,
      ytd_swim_totals: None,
      all_ride_totals: None,
      all_run_totals: None,
      all_swim_totals: None
    }
  }

  pub fn set_biggest_ride_distance(&mut self, biggest_ride_distance: f64) {
    self.biggest_ride_distance = Some(biggest_ride_distance);
  }

  pub fn with_biggest_ride_distance(mut self, biggest_ride_distance: f64) -> ActivityStats {
    self.biggest_ride_distance = Some(biggest_ride_distance);
    self
  }

  pub fn biggest_ride_distance(&self) -> Option<&f64> {
    self.biggest_ride_distance.as_ref()
  }

  pub fn reset_biggest_ride_distance(&mut self) {
    self.biggest_ride_distance = None;
  }

  pub fn set_biggest_climb_elevation_gain(&mut self, biggest_climb_elevation_gain: f64) {
    self.biggest_climb_elevation_gain = Some(biggest_climb_elevation_gain);
  }

  pub fn with_biggest_climb_elevation_gain(mut self, biggest_climb_elevation_gain: f64) -> ActivityStats {
    self.biggest_climb_elevation_gain = Some(biggest_climb_elevation_gain);
    self
  }

  pub fn biggest_climb_elevation_gain(&self) -> Option<&f64> {
    self.biggest_climb_elevation_gain.as_ref()
  }

  pub fn reset_biggest_climb_elevation_gain(&mut self) {
    self.biggest_climb_elevation_gain = None;
  }

  pub fn set_recent_ride_totals(&mut self, recent_ride_totals: ::models::ActivityTotal) {
    self.recent_ride_totals = Some(recent_ride_totals);
  }

  pub fn with_recent_ride_totals(mut self, recent_ride_totals: ::models::ActivityTotal) -> ActivityStats {
    self.recent_ride_totals = Some(recent_ride_totals);
    self
  }

  pub fn recent_ride_totals(&self) -> Option<&::models::ActivityTotal> {
    self.recent_ride_totals.as_ref()
  }

  pub fn reset_recent_ride_totals(&mut self) {
    self.recent_ride_totals = None;
  }

  pub fn set_recent_run_totals(&mut self, recent_run_totals: ::models::ActivityTotal) {
    self.recent_run_totals = Some(recent_run_totals);
  }

  pub fn with_recent_run_totals(mut self, recent_run_totals: ::models::ActivityTotal) -> ActivityStats {
    self.recent_run_totals = Some(recent_run_totals);
    self
  }

  pub fn recent_run_totals(&self) -> Option<&::models::ActivityTotal> {
    self.recent_run_totals.as_ref()
  }

  pub fn reset_recent_run_totals(&mut self) {
    self.recent_run_totals = None;
  }

  pub fn set_recent_swim_totals(&mut self, recent_swim_totals: ::models::ActivityTotal) {
    self.recent_swim_totals = Some(recent_swim_totals);
  }

  pub fn with_recent_swim_totals(mut self, recent_swim_totals: ::models::ActivityTotal) -> ActivityStats {
    self.recent_swim_totals = Some(recent_swim_totals);
    self
  }

  pub fn recent_swim_totals(&self) -> Option<&::models::ActivityTotal> {
    self.recent_swim_totals.as_ref()
  }

  pub fn reset_recent_swim_totals(&mut self) {
    self.recent_swim_totals = None;
  }

  pub fn set_ytd_ride_totals(&mut self, ytd_ride_totals: ::models::ActivityTotal) {
    self.ytd_ride_totals = Some(ytd_ride_totals);
  }

  pub fn with_ytd_ride_totals(mut self, ytd_ride_totals: ::models::ActivityTotal) -> ActivityStats {
    self.ytd_ride_totals = Some(ytd_ride_totals);
    self
  }

  pub fn ytd_ride_totals(&self) -> Option<&::models::ActivityTotal> {
    self.ytd_ride_totals.as_ref()
  }

  pub fn reset_ytd_ride_totals(&mut self) {
    self.ytd_ride_totals = None;
  }

  pub fn set_ytd_run_totals(&mut self, ytd_run_totals: ::models::ActivityTotal) {
    self.ytd_run_totals = Some(ytd_run_totals);
  }

  pub fn with_ytd_run_totals(mut self, ytd_run_totals: ::models::ActivityTotal) -> ActivityStats {
    self.ytd_run_totals = Some(ytd_run_totals);
    self
  }

  pub fn ytd_run_totals(&self) -> Option<&::models::ActivityTotal> {
    self.ytd_run_totals.as_ref()
  }

  pub fn reset_ytd_run_totals(&mut self) {
    self.ytd_run_totals = None;
  }

  pub fn set_ytd_swim_totals(&mut self, ytd_swim_totals: ::models::ActivityTotal) {
    self.ytd_swim_totals = Some(ytd_swim_totals);
  }

  pub fn with_ytd_swim_totals(mut self, ytd_swim_totals: ::models::ActivityTotal) -> ActivityStats {
    self.ytd_swim_totals = Some(ytd_swim_totals);
    self
  }

  pub fn ytd_swim_totals(&self) -> Option<&::models::ActivityTotal> {
    self.ytd_swim_totals.as_ref()
  }

  pub fn reset_ytd_swim_totals(&mut self) {
    self.ytd_swim_totals = None;
  }

  pub fn set_all_ride_totals(&mut self, all_ride_totals: ::models::ActivityTotal) {
    self.all_ride_totals = Some(all_ride_totals);
  }

  pub fn with_all_ride_totals(mut self, all_ride_totals: ::models::ActivityTotal) -> ActivityStats {
    self.all_ride_totals = Some(all_ride_totals);
    self
  }

  pub fn all_ride_totals(&self) -> Option<&::models::ActivityTotal> {
    self.all_ride_totals.as_ref()
  }

  pub fn reset_all_ride_totals(&mut self) {
    self.all_ride_totals = None;
  }

  pub fn set_all_run_totals(&mut self, all_run_totals: ::models::ActivityTotal) {
    self.all_run_totals = Some(all_run_totals);
  }

  pub fn with_all_run_totals(mut self, all_run_totals: ::models::ActivityTotal) -> ActivityStats {
    self.all_run_totals = Some(all_run_totals);
    self
  }

  pub fn all_run_totals(&self) -> Option<&::models::ActivityTotal> {
    self.all_run_totals.as_ref()
  }

  pub fn reset_all_run_totals(&mut self) {
    self.all_run_totals = None;
  }

  pub fn set_all_swim_totals(&mut self, all_swim_totals: ::models::ActivityTotal) {
    self.all_swim_totals = Some(all_swim_totals);
  }

  pub fn with_all_swim_totals(mut self, all_swim_totals: ::models::ActivityTotal) -> ActivityStats {
    self.all_swim_totals = Some(all_swim_totals);
    self
  }

  pub fn all_swim_totals(&self) -> Option<&::models::ActivityTotal> {
    self.all_swim_totals.as_ref()
  }

  pub fn reset_all_swim_totals(&mut self) {
    self.all_swim_totals = None;
  }

}



