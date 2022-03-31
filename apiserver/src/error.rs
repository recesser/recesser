use std::fmt::Debug;

use actix_web::{http, HttpResponse, HttpResponseBuilder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Advertised and computed handle of uploaded do not match.")]
    Integrity,
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
    pub fn integrity(e: impl Debug) -> Self {
        log_original_error(e);
        UserError::Integrity
    }

    pub fn bad_request(e: impl Debug) -> Self {
        log_original_error(e);
        UserError::BadRequest
    }

    pub fn unauthorized(e: impl Debug) -> Self {
        log_original_error(e);
        UserError::Unauthorized
    }

    pub fn not_found(path: &str, e: impl Debug) -> Self {
        log_original_error(e);
        UserError::NotFound {
            path: path.to_string(),
        }
    }

    pub fn internal(e: impl Debug) -> Self {
        log_original_error(e);
        UserError::Internal
    }
}

fn log_original_error(e: impl Debug) {
    tracing::debug!(error = ?e, "Original error");
}

impl actix_web::error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        tracing::debug!(error = ?self);
        HttpResponseBuilder::new(self.status_code())
            .insert_header(http::header::ContentType::plaintext())
            .body(self.to_string())
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            UserError::Integrity => http::StatusCode::BAD_REQUEST,
            UserError::BadRequest => http::StatusCode::BAD_REQUEST,
            UserError::Unauthorized => http::StatusCode::UNAUTHORIZED,
            UserError::NotFound { .. } => http::StatusCode::NOT_FOUND,
            UserError::Internal { .. } => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
