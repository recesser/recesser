mod objectstorage;
mod database;
mod routes;
mod error;

use actix_web::{middleware, App, HttpServer, web};

use objectstorage::ObjectStorage;
use database::Database;

struct AppState {
    objstore: ObjectStorage,
    database: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState { 
                objstore: ObjectStorage::new().unwrap(),
                database: Database::new().unwrap(),
            }))
            .wrap(middleware::Logger::default())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
