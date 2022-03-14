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

    let mut db = app_state.database.clone();

    let object_handle = db
        .get(&handle)
        .await
        .map_err(|e| match e.downcast::<database::KeyNotFoundError>() {
            Ok(e) => UserError::not_found(&format!("artifacts/{}", &e.key), e),
            Err(e) => UserError::internal(e),
        })?
        .object_handle
        .to_string();

    db.delete(&handle).await.map_err(UserError::internal)?;

    log::debug!("Deleted artifact {handle}.");

    let in_use = db
        .search(&object_handle)
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
