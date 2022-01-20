use actix_web::{http, HttpResponse, HttpResponseBuilder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
    #[error("The integrity of the data could not be verified")]
    IntegrityError,
}

impl actix_web::error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(http::header::ContentType::plaintext())
            .body(self.to_string())
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            UserError::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
            UserError::IntegrityError => http::StatusCode::BAD_REQUEST,
        }
    }
}
