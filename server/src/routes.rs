use std::path::{Path, PathBuf};

use actix_multipart::{Field, Multipart};
use actix_web::{get, post, web, Error, HttpResponse};
use anyhow::Result;
use futures_util::TryStreamExt;
use recesser_core::hash::hash_from_disk;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::error::UserError;
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
            "object_id" => {
                advertised_artifact_id = extract_artifact_id(&mut field)
                    .await
                    .map_err(|_| UserError::IntegrityError)?;
            }
            "file" => {
                let mut path = ensure_tmp_dir().await?;
                path.push(Uuid::new_v4().to_string());
                write_to_file(&path, &mut field)
                    .await
                    .map_err(|_| UserError::InternalError)?;

                println!("Uploaded file at path {:?}", &path);
                println!("File content: {}", fs::read_to_string(&path).await?);

                let verified_artifact_id = verify_integrity(&advertised_artifact_id, &path)
                    .await?;

                app_state
                    .objstore
                    .upload_file(verified_artifact_id, &path)
                    .await
                    .map_err(|_| UserError::InternalError)?;

                fs::remove_file(&path).await?;
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

async fn ensure_tmp_dir() -> std::io::Result<PathBuf> {
    let path = PathBuf::from("./tmp");
    if !path.exists() {
        fs::create_dir_all(&path).await?;
    }
    Ok(path)
}

async fn write_to_file(path: &Path, field: &mut Field) -> Result<()> {
    if path.exists() {
        fs::remove_file(&path).await?;
    }

    let mut file = fs::File::create(&path).await?;
    while let Some(chunk) = field.try_next().await? {
        file.write_all(&chunk).await?;
    }
    Ok(())
}

async fn verify_integrity(
    advertised_artifact_id: &Option<String>,
    path: impl AsRef<Path>,
) -> std::result::Result<String, UserError> {
    let determined_artifact_id = hash_from_disk(path).map_err(|_| UserError::IntegrityError)?;
    let advertised_artifact_id = advertised_artifact_id.as_ref().ok_or(UserError::IntegrityError)?;
    
    println!("Advertised ID: {}", &advertised_artifact_id);
    println!("Verified ID: {}", &determined_artifact_id);
    
    if determined_artifact_id.ne(advertised_artifact_id) {
        return Err(UserError::IntegrityError);
    }
    Ok(determined_artifact_id)
}

#[get("/artifacts")]
async fn download() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}
