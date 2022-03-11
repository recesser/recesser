mod delete;
mod download;
mod list;
mod upload;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(upload::upload)
        .service(download::download_file)
        .service(download::download_metadata)
        .service(list::list)
        .service(delete::delete);
}
