#![forbid(unsafe_code)]

mod auth;
mod database;
mod error;
mod objectstorage;
mod routes;
mod settings;

use actix_web::{middleware, web, App, HttpServer};
use recesser_core::user::Scope;

use auth::{HmacKey, Token};
use database::Database;
use objectstorage::ObjectStorage;
use settings::Settings;

pub struct AppState {
    objstore: ObjectStorage,
    database: Database,
    hmac_key: HmacKey,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s = Settings::new().expect("Failed to initialize settings");

    env_logger::Builder::new()
        .parse_filters(&s.log_level)
        .init();

    log::debug!("{s:#?}");

    let rng = ring::rand::SystemRandom::new();
    let hmac_key = HmacKey::new(&rng).expect("Failed to generate HMAC key");
    let initial_token =
        Token::create(Scope::Admin, &hmac_key).expect("Failed to generate initial token");
    log::info!(
        "{}",
        initial_token.to_string().expect("Failed to print token")
    );

    let app_state = web::Data::new(AppState {
        objstore: ObjectStorage::new(&s.objectstorage_addr)
            .await
            .expect("Failed to connect to objectstorage"),
        database: Database::new(&s.database_addr)
            .await
            .expect("Failed to connect to database"),
        hmac_key,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .configure(routes::config)
    })
    .bind(&s.addr)?
    .run()
    .await
}
