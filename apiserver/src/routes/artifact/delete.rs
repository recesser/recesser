use actix_web::{delete, web, Error, HttpResponse};

use crate::database::DocumentNotFoundError;
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
        .map_err(|e| DocumentNotFoundError::downcast(e, &format!("/artifacts/{handle}")))?
        .object_handle
        .to_string();

    metadata_store
        .delete(&handle)
        .await
        .map_err(UserError::internal)?;

    tracing::debug!(%handle, "Deleted artifact");

    let in_use = metadata_store
        .search_object_handle(&object_handle)
        .await
        .map_err(UserError::internal)?;

    if in_use.is_empty() {
        tracing::debug!(%object_handle, "File is orphaned. Deleting it.");
        app_state
            .objstore
            .delete(&object_handle)
            .await
            .map_err(UserError::internal)?;
    } else {
        tracing::debug!(
            %object_handle,
            number_of_artifacts = in_use.len(),
            artifact_handles = ?in_use,
            "Object still referenced by other artifacts",
        )
    }

    Ok(HttpResponse::Accepted().into())
}
