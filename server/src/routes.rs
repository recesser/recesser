mod delete;
mod download;
mod list;
mod upload;

use std::path::Path;

use actix_web::web;
use recesser_core::hash::{verify_file_integrity, verify_integrity};

use crate::error::UserError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(upload::upload)
        .service(download::download)
        .service(list::list)
        .service(delete::delete);
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
