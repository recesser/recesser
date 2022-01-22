use actix_web::{get, web, Error};
use recesser_core::metadata::Metadata;

use crate::error::UserError;
use crate::AppState;

#[get("/")]
async fn list(app_state: web::Data<AppState>) -> Result<web::Json<Vec<Metadata>>, Error> {
    let artifacts = app_state
        .database
        .lock()
        .expect("Failed to lock mutex on database connection.")
        .get_all()
        .await
        .map_err(UserError::internal)?;

    Ok(web::Json(artifacts))
}
