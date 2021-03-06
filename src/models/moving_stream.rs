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
pub struct MovingStream {
  /// The number of data points in this stream
  #[serde(rename = "original_size")]
  original_size: Option<i32>,
  /// The level of detail (sampling) in which this stream was returned
  #[serde(rename = "resolution")]
  resolution: Option<String>,
  /// The base series used in the case the stream was downsampled
  #[serde(rename = "series_type")]
  series_type: Option<String>,
  /// The sequence of moving values for this stream, as boolean values
  #[serde(rename = "data")]
  data: Option<Vec<bool>>
}

impl MovingStream {
  pub fn new() -> MovingStream {
    MovingStream {
      original_size: None,
      resolution: None,
      series_type: None,
      data: None
    }
  }

  pub fn set_original_size(&mut self, original_size: i32) {
    self.original_size = Some(original_size);
  }

  pub fn with_original_size(mut self, original_size: i32) -> MovingStream {
    self.original_size = Some(original_size);
    self
  }

  pub fn original_size(&self) -> Option<&i32> {
    self.original_size.as_ref()
  }

  pub fn reset_original_size(&mut self) {
    self.original_size = None;
  }

  pub fn set_resolution(&mut self, resolution: String) {
    self.resolution = Some(resolution);
  }

  pub fn with_resolution(mut self, resolution: String) -> MovingStream {
    self.resolution = Some(resolution);
    self
  }

  pub fn resolution(&self) -> Option<&String> {
    self.resolution.as_ref()
  }

  pub fn reset_resolution(&mut self) {
    self.resolution = None;
  }

  pub fn set_series_type(&mut self, series_type: String) {
    self.series_type = Some(series_type);
  }

  pub fn with_series_type(mut self, series_type: String) -> MovingStream {
    self.series_type = Some(series_type);
    self
  }

  pub fn series_type(&self) -> Option<&String> {
    self.series_type.as_ref()
  }

  pub fn reset_series_type(&mut self) {
    self.series_type = None;
  }

  pub fn set_data(&mut self, data: Vec<bool>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: Vec<bool>) -> MovingStream {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&Vec<bool>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}



