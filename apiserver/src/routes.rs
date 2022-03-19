mod artifact;
mod repository;
mod user;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/artifacts").configure(artifact::config));
    cfg.service(web::scope("/repositories").configure(repository::config));
    cfg.service(web::scope("/users").configure(user::config));
}
