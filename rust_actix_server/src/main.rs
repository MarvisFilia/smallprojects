use actix_web::{get, App, HttpServer, Result};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

#[get("/{filename:.*}")]
async fn index(info: actix_web::web::Path<(String,)>) -> Result<actix_web::HttpResponse> {
    let file_path: PathBuf = format!("./{}", info.into_inner().0).into();
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return Ok(actix_web::HttpResponse::NotFound().finish()),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(contents))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
