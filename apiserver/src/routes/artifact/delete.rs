use actix_web::{delete, web, Error, HttpResponse};

use crate::database;
use crate::error::UserError;
use crate::AppState;

#[delete("/{handle}")]
async fn delete(
    handle: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let handle = handle.into_inner();

    let metadata_store = &app_state.database.metadata;

    let object_handle = metadata_store
        .retrieve(&handle)
        .await
        .map_err(|e| database::DocumentNotFoundError::downcast(e.into(), "artifacts"))?
        .object_handle
        .to_string();

    metadata_store
        .delete(&handle)
        .await
        .map_err(UserError::internal)?;

    log::debug!("Deleted artifact {handle}.");

    let in_use = metadata_store
        .search_object_handle(&object_handle)
        .await
        .map_err(UserError::internal)?;

    if in_use.is_empty() {
        log::debug!("File {object_handle} is orphaned. Deleting it.");
        app_state
            .objstore
            .delete(&object_handle)
            .await
            .map_err(UserError::internal)?;
    } else {
        log::debug!(
            "File {object_handle} still referenced by {len} artifacts: {in_use:?}",
            len = in_use.len()
        )
    }

    Ok(HttpResponse::Accepted().into())
}
