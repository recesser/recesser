use actix_files::NamedFile;
use actix_web::{get, web, Error};

use super::verify_file;
use crate::database;
use crate::error::UserError;
use crate::AppState;

#[get("/{content_address}")]
async fn download(
    content_address: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<NamedFile, Error> {
    let content_address = content_address.into_inner();

    let mut db = app_state.database.clone();

    let metadata = db.get(&content_address).await.map_err(|e| match e
        .downcast::<database::KeyNotFoundError>()
    {
        Ok(err) => UserError::NotFound {
            path: format!("artifacts/{}", err.key),
        },
        _ => UserError::Internal,
    })?;

    let path = app_state
        .objstore
        .download_file(&metadata.file_content_address)
        .await
        .map_err(UserError::internal)?;

    log::debug!("Path of downloaded file: {path:?}");

    verify_file(&path, &metadata.file_content_address).await?;

    Ok(NamedFile::open_async(&path).await?)
}
