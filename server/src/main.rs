mod database;
mod error;
mod file;
mod objectstorage;
mod routes;

use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};

use database::Database;
use objectstorage::ObjectStorage;

struct AppState {
    objstore: ObjectStorage,
    database: Mutex<Database>,
}

impl AppState {
    async fn new() -> Self {
        AppState {
            objstore: ObjectStorage::new().unwrap(),
            database: Mutex::new(Database::new().await.unwrap()),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let app_state = web::Data::new(AppState::new().await);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
