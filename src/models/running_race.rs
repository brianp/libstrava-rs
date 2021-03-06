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
pub struct RunningRace {
  /// The unique identifier of this race.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// The name of this race.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The type of this race.
  #[serde(rename = "running_race_type")]
  running_race_type: Option<i32>,
  /// The race's distance, in meters.
  #[serde(rename = "distance")]
  distance: Option<f32>,
  /// The time at which the race begins started in the local timezone.
  #[serde(rename = "start_date_local")]
  start_date_local: Option<String>,
  /// The name of the city in which the race is taking place.
  #[serde(rename = "city")]
  city: Option<String>,
  /// The name of the state or geographical region in which the race is taking place.
  #[serde(rename = "state")]
  state: Option<String>,
  /// The name of the country in which the race is taking place.
  #[serde(rename = "country")]
  country: Option<String>,
  /// The set of routes that cover this race's course.
  #[serde(rename = "route_ids")]
  route_ids: Option<Vec<i32>>,
  /// The unit system in which the race should be displayed.
  #[serde(rename = "measurement_preference")]
  measurement_preference: Option<String>,
  /// The vanity URL of this race on Strava.
  #[serde(rename = "url")]
  url: Option<String>,
  /// The URL of this race's website.
  #[serde(rename = "website_url")]
  website_url: Option<String>
}

impl RunningRace {
  pub fn new() -> RunningRace {
    RunningRace {
      id: None,
      name: None,
      running_race_type: None,
      distance: None,
      start_date_local: None,
      city: None,
      state: None,
      country: None,
      route_ids: None,
      measurement_preference: None,
      url: None,
      website_url: None
    }
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> RunningRace {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> RunningRace {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_running_race_type(&mut self, running_race_type: i32) {
    self.running_race_type = Some(running_race_type);
  }

  pub fn with_running_race_type(mut self, running_race_type: i32) -> RunningRace {
    self.running_race_type = Some(running_race_type);
    self
  }

  pub fn running_race_type(&self) -> Option<&i32> {
    self.running_race_type.as_ref()
  }

  pub fn reset_running_race_type(&mut self) {
    self.running_race_type = None;
  }

  pub fn set_distance(&mut self, distance: f32) {
    self.distance = Some(distance);
  }

  pub fn with_distance(mut self, distance: f32) -> RunningRace {
    self.distance = Some(distance);
    self
  }

  pub fn distance(&self) -> Option<&f32> {
    self.distance.as_ref()
  }

  pub fn reset_distance(&mut self) {
    self.distance = None;
  }

  pub fn set_start_date_local(&mut self, start_date_local: String) {
    self.start_date_local = Some(start_date_local);
  }

  pub fn with_start_date_local(mut self, start_date_local: String) -> RunningRace {
    self.start_date_local = Some(start_date_local);
    self
  }

  pub fn start_date_local(&self) -> Option<&String> {
    self.start_date_local.as_ref()
  }

  pub fn reset_start_date_local(&mut self) {
    self.start_date_local = None;
  }

  pub fn set_city(&mut self, city: String) {
    self.city = Some(city);
  }

  pub fn with_city(mut self, city: String) -> RunningRace {
    self.city = Some(city);
    self
  }

  pub fn city(&self) -> Option<&String> {
    self.city.as_ref()
  }

  pub fn reset_city(&mut self) {
    self.city = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> RunningRace {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_country(&mut self, country: String) {
    self.country = Some(country);
  }

  pub fn with_country(mut self, country: String) -> RunningRace {
    self.country = Some(country);
    self
  }

  pub fn country(&self) -> Option<&String> {
    self.country.as_ref()
  }

  pub fn reset_country(&mut self) {
    self.country = None;
  }

  pub fn set_route_ids(&mut self, route_ids: Vec<i32>) {
    self.route_ids = Some(route_ids);
  }

  pub fn with_route_ids(mut self, route_ids: Vec<i32>) -> RunningRace {
    self.route_ids = Some(route_ids);
    self
  }

  pub fn route_ids(&self) -> Option<&Vec<i32>> {
    self.route_ids.as_ref()
  }

  pub fn reset_route_ids(&mut self) {
    self.route_ids = None;
  }

  pub fn set_measurement_preference(&mut self, measurement_preference: String) {
    self.measurement_preference = Some(measurement_preference);
  }

  pub fn with_measurement_preference(mut self, measurement_preference: String) -> RunningRace {
    self.measurement_preference = Some(measurement_preference);
    self
  }

  pub fn measurement_preference(&self) -> Option<&String> {
    self.measurement_preference.as_ref()
  }

  pub fn reset_measurement_preference(&mut self) {
    self.measurement_preference = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> RunningRace {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_website_url(&mut self, website_url: String) {
    self.website_url = Some(website_url);
  }

  pub fn with_website_url(mut self, website_url: String) -> RunningRace {
    self.website_url = Some(website_url);
    self
  }

  pub fn website_url(&self) -> Option<&String> {
    self.website_url.as_ref()
  }

  pub fn reset_website_url(&mut self) {
    self.website_url = None;
  }

}



