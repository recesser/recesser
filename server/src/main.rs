mod objectstorage;
mod routes;

use actix_web::{middleware, App, HttpServer};

use objectstorage::{Backend, Bucket};

struct AppState {
    bucket: Bucket,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(AppState { bucket: Backend::bucket("artifacts").unwrap() })
            .wrap(middleware::Logger::default())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
