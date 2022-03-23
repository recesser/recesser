use actix_files::NamedFile;
use actix_web::{get, web, Error};
use recesser_core::metadata::Metadata;

use crate::database::DocumentNotFoundError;
use crate::error::UserError;
use crate::AppState;

#[get("/{handle}/file")]
async fn download_file(
    handle: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<NamedFile, Error> {
    let handle = handle.into_inner();

    let metadata_store = &app_state.database.metadata;

    let metadata = metadata_store
        .retrieve(&handle)
        .await
        .map_err(|e| DocumentNotFoundError::downcast(e, &format!("/artifacts/{handle}")))?;

    let file = tempfile::NamedTempFile::new()?;
    let filepath = file.path();

    let path = app_state
        .objstore
        .download_file(&metadata.object_handle.to_string(), filepath)
        .await
        .map_err(UserError::internal)?;

    log::debug!("Path of downloaded file: {path:?}");

    Ok(NamedFile::open_async(&filepath).await?)
}

#[get("/{handle}/metadata")]
async fn download_metadata(
    handle: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<web::Json<Metadata>, Error> {
    let handle = handle.into_inner();

    let metadata_store = &app_state.database.metadata;

    let metadata = metadata_store
        .retrieve(&handle)
        .await
        .map_err(|e| DocumentNotFoundError::downcast(e, &format!("/artifacts/{handle}")))?;

    Ok(web::Json(metadata))
}
