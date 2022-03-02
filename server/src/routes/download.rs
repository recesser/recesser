use actix_files::NamedFile;
use actix_web::{get, web, Error};
use recesser_core::metadata::Metadata;

use super::verify_file;
use crate::database;
use crate::error::UserError;
use crate::AppState;

#[get("/{handle}/file")]
async fn download_file(
    handle: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<NamedFile, Error> {
    let handle = handle.into_inner();

    let mut db = app_state.database.clone();

    let metadata =
        db.get(&handle)
            .await
            .map_err(|e| match e.downcast::<database::KeyNotFoundError>() {
                Ok(err) => UserError::NotFound {
                    path: format!("artifacts/{}", err.key),
                },
                _ => UserError::Internal,
            })?;

    let file = tempfile::NamedTempFile::new()?;
    let filepath = file.path();

    let path = app_state
        .objstore
        .download_file(&metadata.file_content_address, filepath)
        .await
        .map_err(UserError::internal)?;

    log::debug!("Path of downloaded file: {path:?}");

    verify_file(filepath, &metadata.file_content_address).await?;

    Ok(NamedFile::open_async(&filepath).await?)
}

#[get("/{handle}/metadata")]
async fn download_metadata(
    handle: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<web::Json<Metadata>, Error> {
    let handle = handle.into_inner();

    let mut db = app_state.database.clone();

    let metadata =
        db.get(&handle)
            .await
            .map_err(|e| match e.downcast::<database::KeyNotFoundError>() {
                Ok(err) => UserError::NotFound {
                    path: format!("artifacts/{}", err.key),
                },
                _ => UserError::Internal,
            })?;

    Ok(web::Json(metadata))
}
