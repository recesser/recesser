use actix_web::{delete, get, post, web, Error};

use crate::auth::{self, Scope};
use crate::error::UserError;
use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(list).service(delete);
}

#[post("")]
async fn create(app_state: web::Data<AppState>) -> Result<String, Error> {
    let token =
        auth::Token::create(Scope::User, &app_state.hmac_key).map_err(UserError::internal)?;
    let serialized_token = token.to_string().map_err(UserError::internal)?;
    Ok(serialized_token)
}

#[get("")]
async fn list(app_state: web::Data<AppState>) -> Result<web::Json<Vec<String>>, Error> {
    Ok(web::Json(vec![String::from("String")]))
}

#[delete("/{id}")]
async fn delete(
    id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<web::Json<Vec<String>>, Error> {
    Ok(web::Json(vec![String::from("String")]))
}
