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
pub struct Zones {
  #[serde(rename = "heart_rate")]
  heart_rate: Option<::models::HeartRateZoneRanges>,
  #[serde(rename = "power")]
  power: Option<::models::PowerZoneRanges>
}

impl Zones {
  pub fn new() -> Zones {
    Zones {
      heart_rate: None,
      power: None
    }
  }

  pub fn set_heart_rate(&mut self, heart_rate: ::models::HeartRateZoneRanges) {
    self.heart_rate = Some(heart_rate);
  }

  pub fn with_heart_rate(mut self, heart_rate: ::models::HeartRateZoneRanges) -> Zones {
    self.heart_rate = Some(heart_rate);
    self
  }

  pub fn heart_rate(&self) -> Option<&::models::HeartRateZoneRanges> {
    self.heart_rate.as_ref()
  }

  pub fn reset_heart_rate(&mut self) {
    self.heart_rate = None;
  }

  pub fn set_power(&mut self, power: ::models::PowerZoneRanges) {
    self.power = Some(power);
  }

  pub fn with_power(mut self, power: ::models::PowerZoneRanges) -> Zones {
    self.power = Some(power);
    self
  }

  pub fn power(&self) -> Option<&::models::PowerZoneRanges> {
    self.power.as_ref()
  }

  pub fn reset_power(&mut self) {
    self.power = None;
  }

}



