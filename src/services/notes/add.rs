use actix_web::{web, post, HttpResponse};
use crate::config::Config;
use crate::{jwt, DbPool, db};
use crate::errors::ApiError;
use crate::jwt::Claims;
use crate::models::Note;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddTemplate {
    pub id: String,
    pub content: String,
    pub last_changed: u64
}

#[post("/note")]
pub async fn add(
    data: web::Json<AddTemplate>,
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let data: AddTemplate = data.into_inner();
    let conn = db_pool.get().expect("Could not get db connection");
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    let note: Note = Note {
        id: data.id,
        account_id: claims.sub,
        content: data.content,
        last_changed: data.last_changed as i64
    };

    web::block(move || db::add_note(&conn, &note))
        .await
        .map_err(|e| ApiError::Duplicate)?;

    Ok(HttpResponse::Ok().body("Created"))
}