#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

mod config;
mod db;
mod errors;
mod jwt;
mod models;
mod schema;
mod services;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel_migrations::*;
use dotenv::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get configuration from .env
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    // Setup database
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let mut conn = pool.get().expect("Could not get connection from pool");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");

    // setup HTTP server
    let server = HttpServer::new(move || {
        let cors = Cors::default();

        App::new()
            .wrap(cors)
            // Pass the config to the handlers
            .data(crate::config::Config::from_env().unwrap())
            // Pass the database pool to the handlers
            .data(pool.clone())
            // Setup the routes for the blobs
            .configure(services::blob::service_config)
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("Server running at http://{}/", config.server_addr.clone());

    server.await
}
