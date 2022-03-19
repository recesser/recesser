use actix_web::{delete, get, put, web, Error};
use recesser_core::repository::{CommitID, Repository};

use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(list)
        .service(show)
        .service(credentials)
        .service(delete);
}

#[put("")]
async fn register(app_state: web::Data<AppState>) -> Result<web::Json<Vec<String>>, Error> {
    Ok(web::Json(vec![String::from("String")]))
}

#[get("")]
async fn list(app_state: web::Data<AppState>) -> Result<web::Json<Vec<String>>, Error> {
    Ok(web::Json(vec![String::from("String")]))
}

#[get("/{name}")]
async fn show(
    name: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<web::Json<Vec<String>>, Error> {
    Ok(web::Json(vec![String::from("String")]))
}

#[get("/{name}/credentials")]
async fn credentials(
    name: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<web::Json<Vec<String>>, Error> {
    Ok(web::Json(vec![String::from("String")]))
}

#[delete("/{name}")]
async fn delete(
    name: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<web::Json<Vec<String>>, Error> {
    Ok(web::Json(vec![String::from("String")]))
}
