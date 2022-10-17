use crate::config::Config;
use crate::errors::ApiError;
use crate::jwt::Claims;
use crate::{db, jwt, DbPool};
use actix_web::{delete, web, HttpResponse};

#[delete("/blob/all")]
pub async fn delete_all(
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ApiError> {
    let mut conn = db_pool.get()?;
    let claims: Claims =
        jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    web::block(move || db::delete_all_blobs(&mut conn, &claims.sub))
        .await
        .map_err(|_| ApiError::NotFound)?;

    Ok(HttpResponse::Ok().body("Deleted"))
}

