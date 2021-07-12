#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod services;
mod models;
mod config;
mod errors;
mod schema;
mod db;
mod jwt;

use actix_web::{HttpServer, middleware, web, App};
use dotenv::dotenv;
use std::ops::DerefMut;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use actix_cors::Cors;
use diesel_migrations::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

embed_migrations!("migrations");

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
    embedded_migrations::run_with_output(&pool.get().expect("Could not get connection from pool"), &mut std::io::stdout());

    // setup HTTP server
    let server = HttpServer::new(move || {
        let cors = Cors::default();

        App::new()
            .wrap(cors)
            // Pass the config to the handlers
            .data(crate::config::Config::from_env().unwrap())
            // Pass the database pool to the handlers
            .data(pool.clone())
            // Setup the routes for the notes
            .configure(services::notes::service_config)
            // Setup the routes for the tags
            .configure(services::tags::service_config)
    })
        .bind(config.server_addr.clone())?
        .run();

    println!("Server running at http://{}/", config.server_addr.clone());

    server.await
}