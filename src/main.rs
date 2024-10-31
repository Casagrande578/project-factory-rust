mod services;
mod schema;
mod model;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== PROJECT-FACTORY : RUNNING ===");

    HttpServer::new(move || { App::new().configure(services::config) }).bind("127.0.0.1:8080")?.run().await
}
