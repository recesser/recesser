use actix_web::http::header;
use actix_web::{delete, get, put, web, Error, HttpRequest, HttpResponse};
use recesser_core::repository::{NewRepository, Repository};
use recesser_core::user::Scope;

use crate::auth::middleware::validate_scope;
use crate::database::DocumentNotFoundError;
use crate::error::UserError;
use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add)
        .service(update_last_commit)
        .service(list)
        .service(show)
        .service(credentials)
        .service(remove);
}

#[put("")]
async fn add(
    new_repository: web::Json<NewRepository>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let new_repository = new_repository.into_inner();

    app_state
        .k8s_apiserver
        .create_ssh_secret(&new_repository.keypair)
        .await
        .map_err(UserError::internal)?;

    let repository = Repository::new(
        &new_repository.name,
        new_repository.keypair.public_key.clone(),
    );
    app_state
        .database
        .repositories
        .add(repository)
        .await
        .map_err(UserError::internal)?;

    app_state
        .secstore
        .store_ssh_key(&new_repository.keypair)
        .await
        .map_err(UserError::internal)?;

    Ok(HttpResponse::Ok().into())
}

#[put("/{organisation}/{repository}/last-commit")]
async fn update_last_commit(
    path: web::Path<(String, String)>,
    body: String,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let name = extract_name(path);

    app_state
        .database
        .repositories
        .update_last_commit(&name, &body)
        .await
        .map_err(|e| DocumentNotFoundError::downcast(e, &format!("/repositories/{name}")))?;

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
        .map_err(|e| DocumentNotFoundError::downcast(e, &format!("/repositories/{name}")))?;

    Ok(web::Json(repository))
}

#[get("/{organisation}/{repository}/credentials")]
async fn credentials(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    validate_scope(&req, Scope::Machine)?;
    let name = extract_name(path);

    let repository = app_state
        .database
        .repositories
        .show(&name)
        .await
        .map_err(|e| DocumentNotFoundError::downcast(e, &format!("/repositories/{name}")))?;
    let fingerprint = repository.public_key.fingerprint.to_string();

    let private_key = app_state
        .secstore
        .get_ssh_key(&fingerprint)
        .await
        .map_err(UserError::internal)?;

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_OCTET_STREAM))
        .body(private_key))
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
        .map_err(|e| DocumentNotFoundError::downcast(e, &format!("/repositories/{name}")))?;

    Ok(HttpResponse::Ok().into())
}

fn extract_name(path: web::Path<(String, String)>) -> String {
    let path = path.into_inner();
    format!("{}/{}", path.0, path.1)
}
