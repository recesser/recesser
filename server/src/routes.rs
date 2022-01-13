use std::path::PathBuf;
use std::str;

use actix_multipart::Multipart;
use actix_web::{get, post, web, Error, HttpResponse};
use anyhow::Result;
use futures_util::TryStreamExt;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(upload).service(download);
}

#[post("/file")]
async fn upload(mut payload: Multipart, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let field_name = content_disposition
            .get_name()
            .ok_or_else(|| HttpResponse::BadRequest().finish())
            .unwrap();

        match field_name {
            "object_id" => {
                let buf: Vec<web::Bytes> = field.try_collect().await?;
                println!("{:?}", str::from_utf8(&buf.concat())?);
            }
            "file" => {
                let mut path = PathBuf::from("./tmp");
                if !path.exists() {
                    fs::create_dir_all(&path).await?;
                }
                path.push(Uuid::new_v4().to_string());
                if path.exists() {
                    fs::remove_file(&path).await?;
                }

                let mut file = fs::File::create(&path).await?;
                while let Some(chunk) = field.try_next().await? {
                    file.write_all(&chunk).await?;
                }

                println!("Uploaded file at path {:?}", &path);
                let out_buf = fs::read_to_string(&path).await?;
                println!("{:?}", out_buf);
                fs::remove_file(&path).await?;

                &app_state.bucket.put_object_blocking("test_file", String::from("Hello").as_bytes()).unwrap();
            }
            _ => println!("Unknown field"),
        }
    }
    Ok(HttpResponse::Ok().into())
}

// fn main() -> Result<()> {
//     let bucket = Backend::bucket("artifacts")?;
//     bucket.put_object_blocking("test_file", String::from("Hello").as_bytes())?;
//     let results = bucket.list_blocking(String::new(), None)?;
//     println!("{:?}", results[0].contents);
//     Ok(())
// }

#[get("/file")]
async fn download() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}
