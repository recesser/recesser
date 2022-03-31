use std::path::PathBuf;
use std::str::FromStr;

use actix_multipart::{Field, Multipart};
use actix_web::{put, web, Error, HttpResponse};
use anyhow::Result;
use futures_util::TryStreamExt;
use recesser_core::handle::Handle;
use recesser_core::metadata::Metadata;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::encryption::{encrypt_file, generate_random_key};
use crate::error::UserError;
use crate::AppState;

#[put("")]
async fn upload(
    mut payload: Multipart,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let metadata_store = &app_state.database.metadata;

    let mut handle: Option<Handle> = None;
    let mut metadata: Option<Metadata> = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition
            .get_name()
            .ok_or(UserError::BadRequest)?;

        match field_name {
            "handle" => {
                handle = extract_handle(&mut field)
                    .await
                    .map_err(UserError::bad_request)?;
            }
            "metadata" => {
                let handle = handle.as_ref().ok_or(UserError::BadRequest)?;
                tracing::debug!(%handle);

                metadata = extract_metadata(&mut field)
                    .await
                    .map_err(UserError::bad_request)?;
            }
            "file" => {
                let handle = handle.as_ref().ok_or(UserError::BadRequest)?;
                let metadata = metadata.as_ref().ok_or(UserError::BadRequest)?;
                tracing::debug!(?metadata);

                let file_exists = app_state
                    .objstore
                    .exists(&metadata.object_handle.to_string())
                    .await
                    .map_err(UserError::internal)?;

                if file_exists {
                    tracing::debug!("File already exist in object storage. Skipping upload.");
                } else {
                    tracing::debug!("File doesn't exist in object storage. Uploading it.");
                    extract_encrypt_and_upload_file(&mut field, metadata, &app_state).await?
                }

                metadata_store
                    .insert(&handle.to_string(), metadata)
                    .await
                    .map_err(UserError::internal)?;
            }
            _ => tracing::debug!(name = field_name, "Unknown field"),
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn extract_handle(field: &mut Field) -> Result<Option<Handle>> {
    let buf = field.try_collect::<Vec<web::Bytes>>().await?.concat();
    let handle = Handle::from_str(&String::from_utf8(buf)?)?;
    Ok(Some(handle))
}

async fn extract_metadata(field: &mut Field) -> Result<Option<Metadata>> {
    let buf = field.try_collect::<Vec<web::Bytes>>().await?.concat();
    Ok(Some(serde_json::from_slice(&buf)?))
}

async fn extract_encrypt_and_upload_file(
    field: &mut Field,
    metadata: &Metadata,
    app_state: &web::Data<AppState>,
) -> std::result::Result<(), UserError> {
    let file = tempfile::NamedTempFile::new().map_err(UserError::internal)?;
    let file_path = file.into_temp_path();

    let computed_object_handle = extract_file(field, file_path.to_path_buf())
        .await
        .map_err(UserError::bad_request)?;
    computed_object_handle
        .verify(&metadata.object_handle)
        .map_err(UserError::integrity)?;

    encrypt_file_and_store_key(
        app_state,
        file_path.to_path_buf(),
        &metadata.object_handle.to_string(),
    )
    .await
    .map_err(UserError::internal)?;

    app_state
        .objstore
        .upload_file(computed_object_handle.to_string(), &file_path)
        .await
        .map_err(UserError::internal)?;

    Ok(())
}

async fn extract_file(field: &mut Field, file_path: PathBuf) -> Result<Handle> {
    let mut file = fs::File::create(&file_path).await?;
    while let Some(chunk) = field.try_next().await? {
        file.write_all(&chunk).await?;
    }
    let handle = web::block(move || Handle::compute_from_file(&file_path)).await??;
    Ok(handle)
}

async fn encrypt_file_and_store_key(
    app_state: &web::Data<AppState>,
    file_path: PathBuf,
    object_handle: &str,
) -> Result<()> {
    let key = generate_random_key(&app_state.rng)?;

    let rng = app_state.rng.clone();
    web::block(move || encrypt_file(&rng, &file_path, &key)).await??;

    app_state
        .secstore
        .store_encryption_key(object_handle, &key)
        .await?;
    Ok(())
}
