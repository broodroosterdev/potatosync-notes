use actix_web::{web, get, HttpResponse};
use crate::config::Config;
use crate::{jwt, DbPool, db};
use crate::errors::ApiError;
use crate::jwt::Claims;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    last_updated: Option<u64>
}

#[get("/")]
pub async fn get(
    config: web::Data<Config>,
    params: web::Query<Params>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let conn = db_pool.get()?;
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;


    let notes = match params.last_updated {
        Some(last_updated) => {
            web::block(move || db::get_notes_updated_after(&conn, last_updated, &claims.sub))
                .await
                .expect("Error")
        },
        None => {
            web::block(move || db::get_notes(&conn, &claims.sub))
               .await
               .map_err(|e| ApiError::NotFound)?
        }
    };

    Ok(HttpResponse::Ok().json(notes))
}