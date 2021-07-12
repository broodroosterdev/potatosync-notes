use actix_web::{web, get, HttpResponse, ResponseError};
use crate::config::Config;
use crate::{jwt, DbPool, db};
use crate::errors::ApiError;
use crate::jwt::Claims;
use actix_web::error::BlockingError;

#[get("/last_changed")]
pub async fn last_changed(
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let conn = db_pool.get().expect("Could not get db connection");
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    let note_change = web::block(move || db::get_last_note_changed(&conn, &claims.sub))
        .await
        .map_err(|e| return match e {
            BlockingError::Error(error) => error,
            BlockingError::Canceled => ApiError::InternalServerError
        })?;



    Ok(HttpResponse::Ok().body("Created"))
}