use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, web, Error};
use anyhow::Result;
use recesser_core::metadata::Metadata;

use crate::database::DocumentNotFoundError;
use crate::encryption::decrypt_file;
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
    let file_path = file.into_temp_path();

    let object_handle_string = metadata.object_handle.to_string();

    app_state
        .objstore
        .download_file(&object_handle_string, &file_path)
        .await
        .map_err(UserError::internal)?;

    get_key_and_decrypt_file(&app_state, &object_handle_string, file_path.to_path_buf())
        .await
        .map_err(UserError::internal)?;

    Ok(NamedFile::open_async(&file_path).await?)
}

async fn get_key_and_decrypt_file(
    app_state: &web::Data<AppState>,
    object_handle: &str,
    file_path: PathBuf,
) -> Result<()> {
    let key_bytes = app_state.secstore.get_encryption_key(object_handle).await?;
    web::block(move || decrypt_file(&file_path, &key_bytes)).await??;
    Ok(())
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
