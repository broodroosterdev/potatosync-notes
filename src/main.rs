mod services;
mod db;
mod auth;

use tonic::{transport::Server, Request, Status};
use notes::notes_server::{NotesServer};
use notes::tags_server::{TagsServer};

use sqlx::postgres::PgPoolOptions;

use crate::services::{MyNotes, MyTags};

pub mod notes {
    tonic::include_proto!("notes"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Could not run dotenv");
    std::env::var("JWT_SECRET").expect("Cant find JWT_SECRET variable");
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5433/notes").await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let addr = "0.0.0.0:50051".parse()?;

    let notes = MyNotes{
        database: pool.clone()
    };

    let tags = MyTags{
        database: pool.clone()
    };

    Server::builder()
        .add_service(NotesServer::with_interceptor(notes, check_auth))
        .add_service(TagsServer::with_interceptor(tags, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

