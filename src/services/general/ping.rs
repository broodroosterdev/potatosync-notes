use actix_web::{web, get, HttpResponse};
use crate::config::Config;
use crate::{DbPool, jwt};
use crate::errors::ApiError;
use crate::jwt::Claims;

#[get("/ping")]
pub async fn ping(
    config: web::Data<Config>,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("Pong!"))
}

#[get("/secure-ping")]
pub async fn secure_ping(
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let _: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;
    Ok(HttpResponse::Ok().body("Pong!"))
}