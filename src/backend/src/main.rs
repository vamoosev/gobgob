use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::fs;
use std::io::Write;
#[actix_web::post("/supersalainen/{filename}")]
async fn post_file(filename: web::Path<String>, body: web::Bytes) -> std::io::Result<HttpResponse> {
    // Write the file to disk
    let mut file = fs::File::create(filename.into_inner())?;
    file.write_all(&body).unwrap();
    
    // Return a success response
    Ok(HttpResponse::Ok().body("File uploaded successfully!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(post_file)
    })
    .bind("127.0.0.1:8080")?
    .run()  
    .await
}