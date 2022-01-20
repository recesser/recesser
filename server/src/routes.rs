use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_multipart::{Field, Multipart};
use actix_web::{get, post, web, Error, HttpResponse};
use anyhow::Result;
use futures_util::TryStreamExt;
use recesser_core::hash::{verify_file_integrity, verify_integrity};
use recesser_core::metadata::Metadata;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::database;
use crate::error::UserError;
use crate::file;
use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(upload).service(download);
}

#[post("/artifacts")]
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

                metadata = extract_and_verify_metadata(&mut field, content_address)
                    .await
                    .map_err(UserError::bad_request)?;
            }
            "file" => {
                let content_address = content_address.as_ref().ok_or(UserError::BadRequest)?;
                let metadata = metadata.as_ref().ok_or(UserError::BadRequest)?;
                log::debug!("Extracted metadata: \n{metadata:#?}");

                let file_path = write_to_file(&mut field)
                    .await
                    .map_err(UserError::internal)?;

                let verified_file_content_address =
                    verify_file(&file_path, &metadata.file_content_address).await?;

                app_state
                    .database
                    .lock()
                    .expect("Failed to lock mutex on database connection.")
                    .set(content_address, &metadata)
                    .await
                    .map_err(UserError::internal)?;

                app_state
                    .objstore
                    .upload_file(&verified_file_content_address, &file_path)
                    .await
                    .map_err(UserError::internal)?;
            }
            _ => log::error!("Unknown field"),
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn extract_string(field: &mut Field) -> Result<Option<String>> {
    let buf = field.try_collect::<Vec<web::Bytes>>().await?.concat();
    Ok(Some(String::from_utf8(buf)?))
}

async fn extract_and_verify_metadata(
    field: &mut Field,
    content_address: &str,
) -> Result<Option<Metadata>> {
    let buf = field.try_collect::<Vec<web::Bytes>>().await?.concat();
    verify(&buf, content_address).await?;
    Ok(Some(serde_json::from_slice(&buf)?))
}

async fn write_to_file(field: &mut Field) -> Result<PathBuf> {
    let file_path = file::tempfile()?;
    let mut file = fs::File::create(&file_path).await?;
    while let Some(chunk) = field.try_next().await? {
        file.write_all(&chunk).await?;
    }
    Ok(file_path)
}

async fn verify_file(
    path: &Path,
    file_content_address: &str,
) -> std::result::Result<String, UserError> {
    let path = path.to_owned();
    let content_address = file_content_address.to_owned();
    let verified_content_address = web::block(move || {
        verify_file_integrity(path, &content_address).expect("Failed to verify file integrity")
    })
    .await
    .map_err(|_| UserError::Integrity)?;
    Ok(verified_content_address)
}

async fn verify(buf: &[u8], content_address: &str) -> std::result::Result<(), UserError> {
    let buf = buf.to_owned();
    let content_address = content_address.to_owned();
    web::block(move || {
        verify_integrity(&buf, &content_address).expect("Failed to verify integrity")
    })
    .await
    .map_err(|_| UserError::Integrity)
}

#[get("/artifacts/{content_address}")]
async fn download(
    content_address: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<NamedFile, Error> {
    let content_address = content_address.into_inner();

    let metadata = app_state
        .database
        .lock()
        .expect("Failed to lock mutex on database connection.")
        .get(&content_address)
        .await
        .map_err(|e| match e.downcast::<database::KeyNotFoundError>() {
            Ok(err) => UserError::NotFound {
                path: format!("artifacts/{}", err.content_address),
            },
            _ => UserError::Internal,
        })?;

    let path = app_state
        .objstore
        .download_file(&metadata.file_content_address)
        .await
        .map_err(UserError::internal)?;

    log::debug!("Downloaded file path: {path:?}");

    verify_file(&path, &content_address).await?;

    Ok(NamedFile::open_async(&path).await?)
}
