mod database;
mod error;
mod filesystem;
mod objectstorage;
mod routes;
mod settings;

use actix_web::{middleware, web, App, HttpServer};

use database::Database;
use objectstorage::ObjectStorage;
use settings::Settings;

struct AppState {
    objstore: ObjectStorage,
    database: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s = Settings::new().expect("Failed to initialize settings");

    env_logger::Builder::new()
        .parse_filters(&s.log_level)
        .init();

    println!("{s:#?}");

    let app_state = web::Data::new(AppState {
        objstore: ObjectStorage::new(&s.objectstorage_addr)
            .expect("Failed to connect to objectstorage"),
        database: Database::new(&s.database_addr)
            .await
            .expect("Failed to connect to database"),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .service(web::scope("/artifacts").configure(routes::config))
    })
    .bind(&s.addr)?
    .run()
    .await
}
