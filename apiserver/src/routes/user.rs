use actix_web::{delete, get, post, web, Error, HttpResponse};

use recesser_core::user::{NewUser, User};

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

    let token = auth::Token::create(new_user.scope.clone(), &app_state.hmac_key)
        .map_err(UserError::internal)?;

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

#[delete("/{id}")]
async fn delete(
    id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();

    app_state
        .database
        .user
        .delete(&id)
        .await
        .map_err(UserError::internal)?;

    Ok(HttpResponse::Ok().into())
}
