use actix_web::{http, HttpResponse, HttpResponseBuilder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Request is not formatted properly.")]
    BadRequest,
    #[error("Access forbidden.")]
    Unauthorized,
    #[error("Resource at {path} doesn't exist.")]
    NotFound { path: String },
    #[error("An internal error occurred. Please try again later.")]
    Internal,
}

impl UserError {
    pub fn internal(e: anyhow::Error) -> Self {
        log::debug!("{e}");
        UserError::Internal
    }

    pub fn unauthorized(e: anyhow::Error) -> Self {
        log::debug!("{e}");
        UserError::Unauthorized
    }

    pub fn not_found(path: &str, e: impl std::error::Error) -> Self {
        log::debug!("{e}");
        UserError::NotFound {
            path: path.to_string(),
        }
    }

    pub fn bad_request(e: anyhow::Error) -> Self {
        log::debug!("{e}");
        UserError::BadRequest
    }
}

impl actix_web::error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        log::debug!("{:#?}", self);
        HttpResponseBuilder::new(self.status_code())
            .insert_header(http::header::ContentType::plaintext())
            .body(self.to_string())
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            UserError::BadRequest => http::StatusCode::BAD_REQUEST,
            UserError::Unauthorized => http::StatusCode::UNAUTHORIZED,
            UserError::NotFound { .. } => http::StatusCode::NOT_FOUND,
            UserError::Internal { .. } => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
