use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};

use sqlx::{sqlite::SqlitePool, Executor, query};


use dotenv::dotenv;
use std::env;


// ? should we use this or db?

// #[actix_web::post("/supersalainen/{filename}")]
// async fn post_file(filename: web::Path<String>, body: web::Bytes) -> std::io::Result<HttpResponse> {
//     let filterred = &filename
//         .into_inner()
//         .chars()
//         .filter(|&c| c != '/' && c != '.')
//         .collect::<String>();
//     let mut file = fs::File::create("./files/".to_string() + filterred)?;
//     file.write_all(&body).unwrap();

//     // Return a success response
//     Ok(HttpResponse::Ok().body(body))
// }

#[actix_web::post("/supersalainen/json")]
async fn postjuttu(db: web::Data<SqlitePool>, body: web::Bytes) -> std::io::Result<HttpResponse> {
    db.execute(query!("INSERT INTO macs (macs) VALUES (?)", body)).await.unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut db = SqlitePool::connect("sqlite:./db.db").await.unwrap();
    dotenv().ok();
    db.execute(query!("CREATE TABLE IF NOT EXISTS macs (macs TEXT)")).await.unwrap();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(postjuttu)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
