mod model;
mod routes;
mod schema;
mod team_services;
mod user_services;
mod projects_services;
mod workitems_services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== PROJECT-FACTORY : RUNNING ===");
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Load environment variables
    dotenv().ok();
    
    // Get database URL and host configuration
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must not be null");
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    // Create database pool with better configuration
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1) // Maintain minimum connections
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(300))
        .connect(database_url.as_str())
        .await
        .expect("Failed to create pool");

    // Configure CORS more securely
    // let cors = Cors::default()
    //     .allowed_origin("*")
    //     .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
    //     .allowed_headers(vec!["Authorization", "Content-Type"])
    //     .max_age(3600);

    println!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(Cors::default()
            .allowed_origin("*")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(3600))
            .wrap(Logger::default())
            .configure(routes::configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}