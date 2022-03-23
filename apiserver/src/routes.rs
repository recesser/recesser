mod artifact;
mod repository;
mod user;

use actix_web::dev::ServiceRequest;
use actix_web::{web, Error, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use recesser_core::user::Scope;

use crate::auth::Token;
use crate::error::UserError;
use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/artifacts")
            .wrap(HttpAuthentication::bearer(validator))
            .configure(artifact::config),
    );
    cfg.service(
        web::scope("/repositories")
            .wrap(HttpAuthentication::bearer(validator))
            .configure(repository::config),
    );
    cfg.service(
        web::scope("/users")
            .wrap(HttpAuthentication::bearer(admin_validator))
            .configure(user::config),
    );
}

pub fn validate_scope(req: HttpRequest, scope: Scope) -> Result<(), UserError> {
    let ext = req.extensions();
    let token = ext.get::<Token>().ok_or(UserError::Internal)?;
    token.validate_scope(scope)
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let app_state = extract_app_state(&req)?;
    let token = validate_token(credentials, app_state)?;
    req.extensions_mut().insert(token);
    Ok(req)
}

async fn admin_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let app_state = extract_app_state(&req)?;
    let token = validate_token(credentials, app_state)?;
    token.validate_scope(Scope::Admin)?;
    Ok(req)
}

fn extract_app_state(req: &ServiceRequest) -> Result<&web::Data<AppState>, UserError> {
    req.app_data::<web::Data<AppState>>()
        .ok_or(UserError::Internal)
}

fn validate_token(
    credentials: BearerAuth,
    app_state: &web::Data<AppState>,
) -> Result<Token, UserError> {
    let token_str = credentials.token();
    log::debug!("{token_str}");
    let hmac_key = app_state.hmac_key.lock().unwrap();
    Token::validate(token_str, &hmac_key).map_err(UserError::unauthorized)
}
