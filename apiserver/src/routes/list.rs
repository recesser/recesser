use actix_web::{get, web, Error};

use crate::error::UserError;
use crate::AppState;

#[get("")]
async fn list(app_state: web::Data<AppState>) -> Result<web::Json<Vec<String>>, Error> {
    let mut db = app_state.database.clone();
    let handles = db.keys().await.map_err(UserError::internal)?;
    Ok(web::Json(handles))
}
