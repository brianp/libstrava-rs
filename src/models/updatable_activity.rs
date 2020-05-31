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
pub struct UpdatableActivity {
  /// Whether this activity is a commute
  #[serde(rename = "commute")]
  commute: Option<bool>,
  /// Whether this activity was recorded on a training machine
  #[serde(rename = "trainer")]
  trainer: Option<bool>,
  /// The description of the activity
  #[serde(rename = "description")]
  description: Option<String>,
  /// The name of the activity
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "type")]
  _type: Option<::models::ActivityType>,
  /// Identifier for the gear associated with the activity. ‘none’ clears gear from activity
  #[serde(rename = "gear_id")]
  gear_id: Option<String>
}

impl UpdatableActivity {
  pub fn new() -> UpdatableActivity {
    UpdatableActivity {
      commute: None,
      trainer: None,
      description: None,
      name: None,
      _type: None,
      gear_id: None
    }
  }

  pub fn set_commute(&mut self, commute: bool) {
    self.commute = Some(commute);
  }

  pub fn with_commute(mut self, commute: bool) -> UpdatableActivity {
    self.commute = Some(commute);
    self
  }

  pub fn commute(&self) -> Option<&bool> {
    self.commute.as_ref()
  }

  pub fn reset_commute(&mut self) {
    self.commute = None;
  }

  pub fn set_trainer(&mut self, trainer: bool) {
    self.trainer = Some(trainer);
  }

  pub fn with_trainer(mut self, trainer: bool) -> UpdatableActivity {
    self.trainer = Some(trainer);
    self
  }

  pub fn trainer(&self) -> Option<&bool> {
    self.trainer.as_ref()
  }

  pub fn reset_trainer(&mut self) {
    self.trainer = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> UpdatableActivity {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> UpdatableActivity {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set__type(&mut self, _type: ::models::ActivityType) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: ::models::ActivityType) -> UpdatableActivity {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&::models::ActivityType> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_gear_id(&mut self, gear_id: String) {
    self.gear_id = Some(gear_id);
  }

  pub fn with_gear_id(mut self, gear_id: String) -> UpdatableActivity {
    self.gear_id = Some(gear_id);
    self
  }

  pub fn gear_id(&self) -> Option<&String> {
    self.gear_id.as_ref()
  }

  pub fn reset_gear_id(&mut self) {
    self.gear_id = None;
  }

}



