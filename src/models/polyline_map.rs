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
pub struct PolylineMap {
  /// The identifier of the map
  #[serde(rename = "id")]
  id: Option<String>,
  /// The polyline of the map, only returned on detailed representation of an object
  #[serde(rename = "polyline")]
  polyline: Option<String>,
  /// The summary polyline of the map
  #[serde(rename = "summary_polyline")]
  summary_polyline: Option<String>
}

impl PolylineMap {
  pub fn new() -> PolylineMap {
    PolylineMap {
      id: None,
      polyline: None,
      summary_polyline: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> PolylineMap {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_polyline(&mut self, polyline: String) {
    self.polyline = Some(polyline);
  }

  pub fn with_polyline(mut self, polyline: String) -> PolylineMap {
    self.polyline = Some(polyline);
    self
  }

  pub fn polyline(&self) -> Option<&String> {
    self.polyline.as_ref()
  }

  pub fn reset_polyline(&mut self) {
    self.polyline = None;
  }

  pub fn set_summary_polyline(&mut self, summary_polyline: String) {
    self.summary_polyline = Some(summary_polyline);
  }

  pub fn with_summary_polyline(mut self, summary_polyline: String) -> PolylineMap {
    self.summary_polyline = Some(summary_polyline);
    self
  }

  pub fn summary_polyline(&self) -> Option<&String> {
    self.summary_polyline.as_ref()
  }

  pub fn reset_summary_polyline(&mut self) {
    self.summary_polyline = None;
  }

}



