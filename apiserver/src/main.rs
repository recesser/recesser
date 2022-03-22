#![forbid(unsafe_code)]

mod auth;
mod database;
mod error;
mod objectstorage;
mod routes;
mod secretstorage;
mod settings;

use actix_web::{middleware, web, App, HttpServer};
use anyhow::{anyhow, Result};
use recesser_core::user::Scope;

use auth::{HmacKey, Token};
use database::Database;
use objectstorage::ObjectStorage;
use secretstorage::SecretStorage;
use settings::Settings;

pub struct AppState {
    objstore: ObjectStorage,
    database: Database,
    secstore: SecretStorage,
    hmac_key: HmacKey,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let s = Settings::new()?;

    env_logger::Builder::new()
        .parse_filters(&s.log_level)
        .init();

    log::debug!("{s:#?}");

    // Initialize object storage
    let objstore = ObjectStorage::new(&s.objectstorage_addr).await?;

    // Initialize database
    let database = Database::new(&s.database_addr).await?;

    // Initialize secret storage
    let vault_token = std::env::var("RECESSER_SECRETSTORAGE_TOKEN")
        .map_err(|_| anyhow!("Secret storage token needs to be specified via environment"))?;
    let secstore = SecretStorage::new(&s.secretstorage_addr, vault_token)?;
    secstore.setup().await?;

    // Initialize HMAC key and access token
    let rng = ring::rand::SystemRandom::new();
    let hmac_key = match secstore.get_hmac_key().await {
        Ok(key_value) => HmacKey::new(&key_value),
        Err(_) => {
            let key_value = HmacKey::generate_key_value(&rng)?;
            let hmac_key = HmacKey::new(&key_value);
            secstore.store_hmac_key(&key_value).await?;
            let initial_token = Token::create(Scope::Admin, &hmac_key)?;
            log::info!("{}", initial_token.to_string()?);
            hmac_key
        }
    };

    let app_state = web::Data::new(AppState {
        objstore,
        database,
        secstore,
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
    .await?;
    Ok(())
}
