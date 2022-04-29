#![forbid(unsafe_code)]

mod auth;
mod database;
mod encryption;
mod error;
mod kubernetes;
mod logging;
mod objectstorage;
mod routes;
mod secretstorage;
mod settings;

use std::str::FromStr;
use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use anyhow::{anyhow, Result};
use recesser_core::user::Scope;
use ring::rand::SystemRandom;
use tracing_subscriber::filter::LevelFilter;

use auth::middleware::validator;
use auth::{HmacKey, Token};
use database::Database;
use objectstorage::ObjectStorage;
use secretstorage::SecretStorage;
use settings::Settings;

pub struct AppState {
    objstore: ObjectStorage,
    database: Database,
    secstore: SecretStorage,
    hmac_key: Mutex<HmacKey>,
    rng: SystemRandom,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let s = Settings::new()?;

    let log_level = LevelFilter::from_str(&s.log_level)?;
    tracing_subscriber::fmt().with_max_level(log_level).init();

    tracing::debug!(settings = ?s);

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
            println!("Initial token: {}", initial_token.to_string()?);
            hmac_key
        }
    };

    let app_state = web::Data::new(AppState {
        objstore,
        database,
        secstore,
        hmac_key: Mutex::new(hmac_key),
        rng,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::config)
            .wrap(logging::init())
            .wrap(HttpAuthentication::bearer(validator))
    })
    .bind(&s.addr)?
    .run()
    .await?;
    Ok(())
}
