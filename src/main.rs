mod services;
mod db;

use tonic::{transport::Server, Request, Response, Status};
use notes::notes_server::{Notes, NotesServer};

use sqlx::postgres::PgPoolOptions;
use sqlx::{Postgres, Pool};
use tonic::metadata::MetadataMap;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::services::MyNotes;


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


    let addr = "0.0.0.0:50051".parse()?;
    let notes = MyNotes{
        database: pool
    };

    Server::builder()
        .add_service(NotesServer::with_interceptor(notes, check_auth))
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

/// JWT claims.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub r#type: String,
    pub exp: usize,
}

fn extract_claims(metadata: &MetadataMap) -> Result<Claims, String> {
    let secret = std::env::var("JWT_SECRET").expect("Cant find JWT_SECRET variable");

    if let Some(jwt) = metadata.get("authorization") {
        return match decode::<Claims>(
            jwt.to_str().unwrap(),
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(tokendata) => Ok(tokendata.claims),
            Err(error) => {
                Err(error.to_string())
            }
        }
    } else {
        Err(String::from("No valid auth token"))
    }
}
