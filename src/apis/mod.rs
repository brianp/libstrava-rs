use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod activities_api;
pub use self::activities_api::{ ActivitiesApi, ActivitiesApiClient };
mod athletes_api;
pub use self::athletes_api::{ AthletesApi, AthletesApiClient };
mod clubs_api;
pub use self::clubs_api::{ ClubsApi, ClubsApiClient };
mod gears_api;
pub use self::gears_api::{ GearsApi, GearsApiClient };
mod routes_api;
pub use self::routes_api::{ RoutesApi, RoutesApiClient };
mod running_races_api;
pub use self::running_races_api::{ RunningRacesApi, RunningRacesApiClient };
mod segment_efforts_api;
pub use self::segment_efforts_api::{ SegmentEffortsApi, SegmentEffortsApiClient };
mod segments_api;
pub use self::segments_api::{ SegmentsApi, SegmentsApiClient };
mod streams_api;
pub use self::streams_api::{ StreamsApi, StreamsApiClient };
mod uploads_api;
pub use self::uploads_api::{ UploadsApi, UploadsApiClient };

pub mod configuration;
pub mod client;
