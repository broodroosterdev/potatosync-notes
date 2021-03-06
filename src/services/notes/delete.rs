use actix_web::{web, delete, HttpResponse};
use crate::config::Config;
use crate::{jwt, DbPool, db};
use crate::errors::ApiError;
use crate::jwt::Claims;

#[delete("/note/{id}")]
pub async fn delete(
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