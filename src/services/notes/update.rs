use actix_web::{web, patch, HttpResponse};
use crate::config::Config;
use crate::{jwt, DbPool, db};
use crate::errors::ApiError;
use crate::jwt::Claims;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NoteTemplate {
    pub content: String,
    pub last_changed: u64
}

#[patch("/note/{id}")]
pub async fn update(
    path: web::Path<(String,)>,
    data: web::Json<NoteTemplate>,
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let data: NoteTemplate = data.into_inner();
    let conn = db_pool.get()?;
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    web::block(move || db::update_note(&conn, &path.into_inner().0, &data, &claims.sub))
        .await
        .map_err(|e| ApiError::NotFound)?;

    Ok(HttpResponse::Ok().body("Updated"))
}
