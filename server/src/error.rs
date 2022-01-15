use actix_web::{error, http::header, http::StatusCode, HttpResponse, HttpResponseBuilder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
    #[error("The integrity of the data could not be verified")]
    IntegrityError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType::plaintext())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::IntegrityError => StatusCode::BAD_REQUEST,
        }
    }
}
