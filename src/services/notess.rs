use actix_web::{post, patch, delete, get, web, HttpResponse, Error};
use crate::models::Note;
use uuid::Uuid;
use serde::Deserialize;
use crate::{db, DbPool, jwt};
use crate::errors::ApiError;
use crate::jwt::Claims;
use crate::config::Config;

#[derive(Deserialize)]
pub struct AddTemplate {
    pub id: String,
    pub content: String
}

#[post("/note")]
async fn add(
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
        account_id: claims.sub.clone(),
        content: data.content
    };

    web::block(move || db::add_note(&conn, &note))
        .await
        .map_err(|e| ApiError::NotFound)?;

    Ok(HttpResponse::Ok().body("Created"))
}

#[derive(Deserialize)]
pub struct UpdateTemplate {
    pub content: String
}

#[patch("/note/{id}")]
async fn update(
    path: web::Path<(String,)>,
    data: web::Json<UpdateTemplate>,
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let data: UpdateTemplate = data.into_inner();
    let conn = db_pool.get()?;
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    web::block(move || db::update_note(&conn, &path.into_inner().0, &data.content, &claims.sub))
        .await
        .map_err(|e| ApiError::NotFound)?;

    Ok(HttpResponse::Ok().body("Updated"))
}

#[delete("/note/{id}")]
async fn delete(
    path: web::Path<(String,)>,
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let conn = db_pool.get()?;
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    web::block(move || db::delete_note(&conn, &path.into_inner().0, &claims.sub))
        .await
        .map_err(|e| ApiError::NotFound)?;

    Ok(HttpResponse::Ok().body("Deleted"))
}

#[get("/note")]
async fn get(
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let conn = db_pool.get()?;
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    let notes = web::block(move || db::get_notes(&conn, &claims.sub))
        .await
        .map_err(|e| ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(notes))
}

// Configure files app services
pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(add)       // POST /note
        .service(update)    // PATCH /note/{id}
        .service(delete)    // DELETE /note/{id}
        .service(get);                      // GET /note
}