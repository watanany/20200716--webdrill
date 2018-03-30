use std::result;
use std::convert::From;
use reqwest;
use serde_json;
mod status;
pub use self::status::Status;

#[derive(Debug)]
pub enum Error {
    NotImplemented,
    ParseJson(serde_json::Error),
    Request(reqwest::Error),
    Url(reqwest::UrlError),
    Response(Status),
}

pub type Result<T> = result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(error)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(error: reqwest::UrlError) -> Self {
        Error::Url(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::ParseJson(error)
    }
}
