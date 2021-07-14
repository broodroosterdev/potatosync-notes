use actix_web::{web, delete, HttpResponse};
use crate::config::Config;
use crate::{jwt, DbPool, db};
use crate::errors::ApiError;
use crate::jwt::Claims;

#[delete("/all")]
pub async fn delete_all(
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let conn = db_pool.get()?;
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    web::block(move || db::delete_all_note(&conn, &claims.sub))
        .await
        .map_err(|e| ApiError::NotFound)?;

    Ok(HttpResponse::Ok().body("Deleted"))
}