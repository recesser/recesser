use std::path::Path;

use actix_files::NamedFile;
use actix_multipart::{Field, Multipart};
use actix_web::{get, post, web, Error, HttpResponse};
use anyhow::Result;
use futures_util::TryStreamExt;
use recesser_core::hash::verify_integrity;
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
    let mut advertised_artifact_id: Option<String> = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let field_name = content_disposition
            .get_name()
            .ok_or_else(|| HttpResponse::BadRequest().finish())
            .unwrap();

        match field_name {
            "artifact_id" => {
                advertised_artifact_id = extract_artifact_id(&mut field)
                    .await
                    .map_err(|_| UserError::IntegrityError)?;
            }
            "file" => {
                let path = file::tempfile()?;
                write_to_file(&path, &mut field)
                    .await
                    .map_err(|_| UserError::InternalError)?;

                let advertised_artifact_id = advertised_artifact_id
                    .as_ref()
                    .ok_or(UserError::IntegrityError)?;
                let verified_artifact_id =
                    verify_artifact_id(&path, advertised_artifact_id).await?;

                app_state
                    .database
                    .lock()
                    .unwrap()
                    .set()
                    .await
                    .map_err(|_| UserError::InternalError)?;

                app_state
                    .objstore
                    .upload_file(verified_artifact_id, &path)
                    .await
                    .map_err(|_| UserError::InternalError)?;
            }
            _ => println!("Unknown field"),
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn extract_artifact_id(field: &mut Field) -> Result<Option<String>> {
    let buf: Vec<web::Bytes> = field.try_collect().await?;
    let advertised_artifact_id = String::from_utf8(buf.concat())?;
    Ok(Some(advertised_artifact_id))
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
            println!("{}", e);
            UserError::InternalError
        })?;

    // let contents = fs::read_to_string(&path).await?;
    // println!("Content of file at path: {:?}: {}", &path, &contents);
    println!("Path: {:?}", &path);

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
        println!("{}", e);
        UserError::IntegrityError
    })?;
    Ok(verified_artifact_id)
}
