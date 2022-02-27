mod services;
mod db;
mod auth;

use tonic::{transport::Server, Request, Status};
use blob::blob_server::{BlobServer};

use sqlx::postgres::PgPoolOptions;
use crate::services::BlobService;

pub mod blob {
    tonic::include_proto!("blob"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Could not run dotenv");
    //std::env::var("JWT_SECRET").expect("Cant find JWT_SECRET variable");
    let database_url = std::env::var("DATABASE_URL").expect("Cant find DATABASE_URL variable");
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let addr = "0.0.0.0:4000".parse()?;

    let blobs = BlobService{
        database: pool.clone()
    };

    Server::builder()
        .add_service(BlobServer::with_interceptor(blobs, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    return Ok(req);
    match req.metadata().get("authorization") {
        Some(t) => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

