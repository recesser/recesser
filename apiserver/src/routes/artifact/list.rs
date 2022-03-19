use actix_web::{get, web, Error};

use crate::error::UserError;
use crate::AppState;

#[get("")]
async fn list(app_state: web::Data<AppState>) -> Result<web::Json<Vec<String>>, Error> {
    let metadata_store = &app_state.database.metadata;
    let handles = metadata_store
        .list_handles()
        .await
        .map_err(UserError::internal)?;
    Ok(web::Json(handles))
}
