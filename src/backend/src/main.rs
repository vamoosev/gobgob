use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, http::ConnectionType};

use sqlite::{self, Connection};
use dotenv::dotenv;

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

use tokio::sync::Mutex;
use std::sync::Arc;

#[actix_web::post("/supersalainen/json")]
async fn postjuttu(db: web::Data<Arc<Mutex<Connection>>>, body: String) -> std::io::Result<HttpResponse> {
    let db = db.lock().await;
    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // pass db to app
    let db = sqlite::open("./db.db").unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS juttu (juttu TEXT)").unwrap();
    let db = Arc::new(Mutex::new(db));
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(postjuttu)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
