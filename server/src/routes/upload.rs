use std::path::PathBuf;

use actix_multipart::{Field, Multipart};
use actix_web::{post, web, Error, HttpResponse};
use anyhow::Result;
use futures_util::TryStreamExt;
use recesser_core::metadata::Metadata;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use super::{verify, verify_file};
use crate::error::UserError;
use crate::filesystem::tempfile;
use crate::AppState;

#[post("")]
async fn upload(
    mut payload: Multipart,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut content_address: Option<String> = None;
    let mut metadata: Option<Metadata> = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition
            .get_name()
            .ok_or(UserError::BadRequest)?;

        match field_name {
            "content-address" => {
                content_address = extract_string(&mut field)
                    .await
                    .map_err(UserError::bad_request)?;
            }
            "metadata" => {
                let content_address = content_address.as_ref().ok_or(UserError::BadRequest)?;
                log::debug!("Extracted content-address: {content_address}");

                let buf = field.try_collect::<Vec<web::Bytes>>().await?.concat();
                verify(&buf, content_address).await?;
                metadata = Some(serde_json::from_slice(&buf).map_err(|_| UserError::BadRequest)?);
            }
            "file" => {
                let content_address = content_address.as_ref().ok_or(UserError::BadRequest)?;
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
                    upload_file(&mut field, metadata, &app_state)
                        .await
                        .map_err(UserError::internal)?
                }

                app_state
                    .database
                    .set(content_address, &metadata)
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

async fn upload_file(
    field: &mut Field,
    metadata: &Metadata,
    app_state: &web::Data<AppState>,
) -> Result<()> {
    let file_path = write_to_file(field).await?;

    let verified_file_content_address =
        verify_file(&file_path, &metadata.file_content_address).await?;

    app_state
        .objstore
        .upload_file(&verified_file_content_address, &file_path)
        .await?;
    Ok(())
}

async fn write_to_file(field: &mut Field) -> Result<PathBuf> {
    let file_path = tempfile()?;
    let mut file = fs::File::create(&file_path).await?;
    while let Some(chunk) = field.try_next().await? {
        file.write_all(&chunk).await?;
    }
    Ok(file_path)
}
