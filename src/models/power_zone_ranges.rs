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
pub struct PowerZoneRanges {
  #[serde(rename = "zones")]
  zones: Option<::models::ZoneRanges>
}

impl PowerZoneRanges {
  pub fn new() -> PowerZoneRanges {
    PowerZoneRanges {
      zones: None
    }
  }

  pub fn set_zones(&mut self, zones: ::models::ZoneRanges) {
    self.zones = Some(zones);
  }

  pub fn with_zones(mut self, zones: ::models::ZoneRanges) -> PowerZoneRanges {
    self.zones = Some(zones);
    self
  }

  pub fn zones(&self) -> Option<&::models::ZoneRanges> {
    self.zones.as_ref()
  }

  pub fn reset_zones(&mut self) {
    self.zones = None;
  }

}



