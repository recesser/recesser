use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage};
use tracing::Span;
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder, TracingLogger};

use crate::auth::Token;

pub struct CustomRootSpanBuilder;

impl RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(req: &ServiceRequest) -> Span {
        let user_id: String = match req.extensions().get::<Token>() {
            Some(token) => token.user_id().into(),
            None => "Not provided".into(),
        };
        tracing::info_span!("Request", %user_id)
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}

pub fn init() -> TracingLogger<CustomRootSpanBuilder> {
    TracingLogger::<CustomRootSpanBuilder>::new()
}
