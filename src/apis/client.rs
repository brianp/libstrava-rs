use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  activities_api: Box<::apis::ActivitiesApi>,
  athletes_api: Box<::apis::AthletesApi>,
  clubs_api: Box<::apis::ClubsApi>,
  gears_api: Box<::apis::GearsApi>,
  routes_api: Box<::apis::RoutesApi>,
  running_races_api: Box<::apis::RunningRacesApi>,
  segment_efforts_api: Box<::apis::SegmentEffortsApi>,
  segments_api: Box<::apis::SegmentsApi>,
  streams_api: Box<::apis::StreamsApi>,
  uploads_api: Box<::apis::UploadsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      activities_api: Box::new(::apis::ActivitiesApiClient::new(rc.clone())),
      athletes_api: Box::new(::apis::AthletesApiClient::new(rc.clone())),
      clubs_api: Box::new(::apis::ClubsApiClient::new(rc.clone())),
      gears_api: Box::new(::apis::GearsApiClient::new(rc.clone())),
      routes_api: Box::new(::apis::RoutesApiClient::new(rc.clone())),
      running_races_api: Box::new(::apis::RunningRacesApiClient::new(rc.clone())),
      segment_efforts_api: Box::new(::apis::SegmentEffortsApiClient::new(rc.clone())),
      segments_api: Box::new(::apis::SegmentsApiClient::new(rc.clone())),
      streams_api: Box::new(::apis::StreamsApiClient::new(rc.clone())),
      uploads_api: Box::new(::apis::UploadsApiClient::new(rc.clone())),
    }
  }

  pub fn activities_api(&self) -> &::apis::ActivitiesApi{
    self.activities_api.as_ref()
  }

  pub fn athletes_api(&self) -> &::apis::AthletesApi{
    self.athletes_api.as_ref()
  }

  pub fn clubs_api(&self) -> &::apis::ClubsApi{
    self.clubs_api.as_ref()
  }

  pub fn gears_api(&self) -> &::apis::GearsApi{
    self.gears_api.as_ref()
  }

  pub fn routes_api(&self) -> &::apis::RoutesApi{
    self.routes_api.as_ref()
  }

  pub fn running_races_api(&self) -> &::apis::RunningRacesApi{
    self.running_races_api.as_ref()
  }

  pub fn segment_efforts_api(&self) -> &::apis::SegmentEffortsApi{
    self.segment_efforts_api.as_ref()
  }

  pub fn segments_api(&self) -> &::apis::SegmentsApi{
    self.segments_api.as_ref()
  }

  pub fn streams_api(&self) -> &::apis::StreamsApi{
    self.streams_api.as_ref()
  }

  pub fn uploads_api(&self) -> &::apis::UploadsApi{
    self.uploads_api.as_ref()
  }


}
