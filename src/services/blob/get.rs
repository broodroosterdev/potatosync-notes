use crate::config::Config;
use crate::errors::ApiError;
use crate::jwt::Claims;
use crate::{db, jwt, DbPool};
use actix_web::{get, web, HttpResponse};

#[get("/blob")]
pub async fn get(
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ApiError> {
    let mut conn = db_pool.get()?;
    let claims: Claims =
        jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    let blobs = web::block(move || db::get_blobs(&mut conn, &claims.sub))
        .await
        .map_err(|_| ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(blobs))
}

