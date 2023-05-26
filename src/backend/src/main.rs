use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};
use pcap_file::pcap::PcapReader;
use std::fs;
use std::io::{Chain, Read, Write};

#[actix_web::post("/supersalainen/{filename}")]
async fn post_file(filename: web::Path<String>, body: web::Bytes) -> std::io::Result<HttpResponse> {
    let filterred = &filename
        .into_inner()
        .chars()
        .filter(|&c| c != '/' && c != '.')
        .collect::<String>();
    let mut file = fs::File::create("./files/".to_string() + filterred)?;
    file.write_all(&body).unwrap();

    // Return a success response
    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(post_file)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
