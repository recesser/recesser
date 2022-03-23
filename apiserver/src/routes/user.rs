use actix_web::{delete, get, post, web, Error};
use recesser_core::user::{NewUser, Scope, User};

use crate::auth;
use crate::error::UserError;
use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(list).service(delete);
}

#[post("")]
async fn create(
    new_user: web::Json<NewUser>,
    app_state: web::Data<AppState>,
) -> Result<String, Error> {
    let new_user = new_user.into_inner();

    let hmac_key = app_state.hmac_key.lock().unwrap();
    let token =
        auth::Token::create(new_user.scope.clone(), &hmac_key).map_err(UserError::internal)?;

    app_state
        .database
        .user
        .create(&token.extract_user())
        .await
        .map_err(UserError::internal)?;

    let serialized_token = token.to_string().map_err(UserError::internal)?;
    Ok(serialized_token)
}

#[get("")]
async fn list(app_state: web::Data<AppState>) -> Result<web::Json<Vec<User>>, Error> {
    let users = app_state
        .database
        .user
        .list()
        .await
        .map_err(UserError::internal)?;
    Ok(web::Json(users))
}

#[delete("")]
async fn delete(app_state: web::Data<AppState>) -> Result<String, Error> {
    // Delete all user records
    app_state
        .database
        .user
        .delete()
        .await
        .map_err(UserError::internal)?;

    // Generate new HMAC key and root token
    let key_value =
        auth::HmacKey::generate_key_value(&app_state.rng).map_err(UserError::internal)?;
    let hmac_key = auth::HmacKey::new(&key_value);
    let token = auth::Token::create(Scope::Admin, &hmac_key).map_err(UserError::internal)?;
    let serialized_token = token.to_string().map_err(UserError::internal)?;

    // Overwrite old key in secret storage and app_state
    app_state
        .secstore
        .store_hmac_key(&key_value)
        .await
        .map_err(UserError::internal)?;
    *app_state.hmac_key.lock().unwrap() = hmac_key;

    Ok(serialized_token)
}
