mod model;
mod routes;
mod schema;
mod team_services;
mod user_services;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== PROJECT-FACTORY : RUNNING ===");

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must not be null");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
    {
        Ok(pool) => {
            println!("Connected to DB");
            pool
        }
        Err(error) => {
            println!("Failed to connect to the DB {:?}", error);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
