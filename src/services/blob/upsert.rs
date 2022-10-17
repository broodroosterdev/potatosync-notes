use crate::config::Config;
use crate::errors::ApiError;
use crate::jwt::Claims;
use crate::models::Blob;
use crate::{db, jwt, DbPool};
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddTemplate {
    pub id: String,
    pub content: String,
    pub last_changed: u64,
}

#[post("/blob")]
pub async fn upsert(
    data: web::Json<AddTemplate>,
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ApiError> {
    let data: AddTemplate = data.into_inner();
    let mut conn = db_pool.get().expect("Could not get db connection");
    let claims: Claims =
        jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    let blob: Blob = Blob {
        id: data.id,
        account_id: claims.sub,
        content: data.content,
        last_changed: data.last_changed as i64,
    };

    web::block(move || db::upsert_blob(&mut conn, &blob))
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().body("Created or Updated"))
}

