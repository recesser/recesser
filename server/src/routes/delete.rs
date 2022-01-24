use actix_web::{delete, web, Error, HttpResponse};

use crate::database;
use crate::error::UserError;
use crate::AppState;

#[delete("/{content_address}")]
async fn delete(
    content_address: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let content_address = content_address.into_inner();

    app_state
        .database
        .delete(&content_address)
        .await
        .map_err(|e| match e.downcast::<database::KeyNotFoundError>() {
            Ok(err) => UserError::NotFound {
                path: format!("artifacts/{}", err.key),
            },
            _ => UserError::Internal,
        })?;

    // TODO: Implement garbage collection of objects in objectstorage

    Ok(HttpResponse::Accepted().into())
}
