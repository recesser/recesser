use actix_web::{delete, get, put, web, Error, HttpRequest, HttpResponse};
use recesser_core::repository::{NewRepository, Repository};
use recesser_core::user::Scope;

use crate::database;
use crate::error::UserError;
use crate::routes::validate_scope;
use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add)
        .service(list)
        .service(show)
        .service(credentials)
        .service(remove);
}

#[put("")]
async fn add(
    new_user: web::Json<NewRepository>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let new_repository = new_user.into_inner();
    let repository = Repository::from_new_repository(new_repository);

    app_state
        .database
        .repositories
        .add(repository)
        .await
        .map_err(UserError::internal)?;

    Ok(HttpResponse::Ok().into())
}

#[get("")]
async fn list(app_state: web::Data<AppState>) -> Result<web::Json<Vec<Repository>>, Error> {
    let repositories = app_state
        .database
        .repositories
        .list()
        .await
        .map_err(UserError::internal)?;
    Ok(web::Json(repositories))
}

#[get("/{organisation}/{repository}")]
async fn show(
    path: web::Path<(String, String)>,
    app_state: web::Data<AppState>,
) -> Result<web::Json<Repository>, Error> {
    let name = extract_name(path);

    let repository = app_state
        .database
        .repositories
        .show(&name)
        .await
        .map_err(|e| database::DocumentNotFoundError::downcast(e.into(), "repositories"))?;

    Ok(web::Json(repository))
}

#[get("/{organisation}/{repository}/credentials")]
async fn credentials(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    _app_state: web::Data<AppState>,
) -> Result<web::Json<Vec<String>>, Error> {
    validate_scope(req, Scope::Machine)?;
    let _name = extract_name(path);
    Ok(web::Json(vec![String::from("Not implemented")]))
}

#[delete("/{organisation}/{repository}")]
async fn remove(
    path: web::Path<(String, String)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let name = extract_name(path);

    app_state
        .database
        .repositories
        .remove(&name)
        .await
        .map_err(|e| database::DocumentNotFoundError::downcast(e.into(), "repositories"))?;

    Ok(HttpResponse::Ok().into())
}

fn extract_name(path: web::Path<(String, String)>) -> String {
    let path = path.into_inner();
    format!("{}/{}", path.0, path.1)
}
