use std::path::Path;

use actix_multipart::{Field, Multipart};
use actix_web::{put, web, Error, HttpResponse};
use anyhow::Result;
use futures_util::TryStreamExt;
use recesser_core::metadata::Metadata;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::error::UserError;
use crate::AppState;

#[put("")]
async fn upload(
    mut payload: Multipart,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut db = app_state.database.clone();

    let mut handle: Option<String> = None;
    let mut metadata: Option<Metadata> = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition
            .get_name()
            .ok_or(UserError::BadRequest)?;

        match field_name {
            "handle" => {
                handle = extract_string(&mut field)
                    .await
                    .map_err(UserError::bad_request)?;
            }
            "metadata" => {
                let handle = handle.as_ref().ok_or(UserError::BadRequest)?;
                log::debug!("Extracted handle: {handle}");

                let buf = field.try_collect::<Vec<web::Bytes>>().await?.concat();
                metadata = Some(serde_json::from_slice(&buf).map_err(|_| UserError::BadRequest)?);
            }
            "file" => {
                let handle = handle.as_ref().ok_or(UserError::BadRequest)?;
                let metadata = metadata.as_ref().ok_or(UserError::BadRequest)?;
                log::debug!("Extracted metadata: \n{metadata:#?}");

                let file_exists = app_state
                    .objstore
                    .exists(&metadata.file_content_address)
                    .await
                    .map_err(UserError::internal)?;

                if file_exists {
                    log::debug!("File already exist in object storage. Skipping upload.");
                } else {
                    log::debug!("File doesn't exist in object storage. Uploading it.");
                    extract_and_upload_file(&mut field, metadata, &app_state)
                        .await
                        .map_err(UserError::internal)?
                }

                db.set(handle, &metadata)
                    .await
                    .map_err(UserError::internal)?;
            }
            _ => log::error!("Unknown field: {field_name}"),
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn extract_string(field: &mut Field) -> Result<Option<String>> {
    let buf = field.try_collect::<Vec<web::Bytes>>().await?.concat();
    Ok(Some(String::from_utf8(buf)?))
}

async fn extract_and_upload_file(
    field: &mut Field,
    metadata: &Metadata,
    app_state: &web::Data<AppState>,
) -> Result<()> {
    let file = tempfile::NamedTempFile::new()?;
    let filepath = file.path();
    extract_file(field, filepath).await?;

    app_state
        .objstore
        .upload_file(&metadata.file_content_address, &filepath)
        .await?;
    Ok(())
}

async fn extract_file(field: &mut Field, file_path: &Path) -> Result<()> {
    let mut file = fs::File::create(&file_path).await?;
    while let Some(chunk) = field.try_next().await? {
        file.write_all(&chunk).await?;
    }
    Ok(())
}
