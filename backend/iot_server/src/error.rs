use actix_web::{error::BlockingError, HttpResponse, ResponseError};
use derive_more::{Display, From};
use std::error::Error;

#[derive(Debug, Display, From)]
pub enum APIError {
  #[display(fmt = "Internal Server Error")]
  InternalServerError,
}

impl Error for APIError {}

impl ResponseError for APIError {
  fn error_response(&self) -> HttpResponse {
    match &*self {
      APIError::InternalServerError => {
        HttpResponse::InternalServerError().body(format!("{}", self))
      }
    }
  }
}

impl From<BlockingError<APIError>> for APIError {
  fn from(error: BlockingError<APIError>) -> APIError {
    match error {
      BlockingError::Error(err) => err,
      BlockingError::Canceled => APIError::InternalServerError,
    }
  }
}

//generate_internal_server_error!(IOError);
