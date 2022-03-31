mod artifact;
mod repository;
mod user;

use actix_web::dev::Service;
use actix_web::web;
use recesser_core::user::Scope;

use crate::auth::middleware::validate_scope;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/artifacts").configure(artifact::config));
    cfg.service(web::scope("/repositories").configure(repository::config));
    cfg.service(
        web::scope("/users")
            .configure(user::config)
            .wrap_fn(|req, srv| {
                let result = validate_scope(&req, Scope::Admin);
                let fut = srv.call(req);
                async {
                    result?;
                    fut.await
                }
            }),
    );
}
