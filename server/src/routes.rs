use std::path::Path;

use actix_files::NamedFile;
use actix_multipart::{Field, Multipart};
use actix_web::{get, post, web, Error, HttpResponse};
use anyhow::Result;
use futures_util::TryStreamExt;
use recesser_core::hash::verify_integrity;
use recesser_core::metadata::Metadata;
use tokio::fs;
use tokio::io::AsyncWriteExt;

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
    let mut metadata: Option<Metadata> = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let field_name = content_disposition
            .get_name()
            .ok_or_else(|| HttpResponse::BadRequest().finish())
            .unwrap();

        match field_name {
            "metadata" => {
                metadata = extract_metadata(&mut field).await.map_err(|e| {
                    log::debug!("{}", e);
                    UserError::IntegrityError
                })?;
                log::debug!("Extracted metadata: {:?}", &metadata);
            }
            "file" => {
                let metadata = metadata.as_ref().ok_or(UserError::IntegrityError)?;

                let path = file::tempfile()?;
                write_to_file(&path, &mut field).await.map_err(|e| {
                    log::debug!("{}", e);
                    UserError::IntegrityError
                })?;

                let verified_artifact_id = verify_artifact_id(&path, &metadata.artifact_id).await?;

                app_state
                    .database
                    .lock()
                    .unwrap()
                    .set(&metadata.artifact_id, &metadata)
                    .await
                    .map_err(|e| {
                        log::debug!("{}", e);
                        UserError::IntegrityError
                    })?;

                app_state
                    .objstore
                    .upload_file(verified_artifact_id, &path)
                    .await
                    .map_err(|e| {
                        log::debug!("{}", e);
                        UserError::IntegrityError
                    })?;
            }
            _ => log::error!("Unknown field"),
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn extract_metadata(field: &mut Field) -> Result<Option<Metadata>> {
    let buf: Vec<web::Bytes> = field.try_collect().await?;
    let metadata: Metadata = serde_json::from_slice(&buf.concat())?;
    Ok(Some(metadata))
}

async fn write_to_file(path: &Path, field: &mut Field) -> Result<()> {
    let mut file = fs::File::create(&path).await?;
    while let Some(chunk) = field.try_next().await? {
        file.write_all(&chunk).await?;
    }
    Ok(())
}

#[get("/artifacts/{id}")]
async fn download(
    artifact_id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<NamedFile, Error> {
    let artifact_id = artifact_id.into_inner();

    let path = app_state
        .objstore
        .download_file(&artifact_id)
        .await
        .map_err(|e| {
            log::debug!("{}", e);
            UserError::InternalError
        })?;

    log::trace!("Uploaded file path: {:?}", &path);

    verify_artifact_id(&path, &artifact_id).await?;

    Ok(NamedFile::open_async(&path).await?)
}

async fn verify_artifact_id(
    path: &Path,
    advertised_artifact_id: &str,
) -> std::result::Result<String, UserError> {
    let copied_path = path.to_owned();
    let copied_advertised_artifact_id = advertised_artifact_id.to_owned();
    let verified_artifact_id = web::block(move || {
        verify_integrity(copied_path, &copied_advertised_artifact_id)
            .expect("Failed to verify integrity")
    })
    .await
    .map_err(|e| {
        log::debug!("{}", e);
        UserError::IntegrityError
    })?;
    Ok(verified_artifact_id)
}
