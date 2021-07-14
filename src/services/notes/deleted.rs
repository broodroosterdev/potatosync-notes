use actix_web::{web, post, HttpResponse};
use crate::config::Config;
use crate::{jwt, DbPool, db};
use crate::errors::ApiError;
use crate::jwt::Claims;
use serde::Deserialize;

#[post("/deleted")]
pub async fn deleted(
    data: web::Json<Vec<String>>,
    config: web::Data<Config>,
    jwt: jwt::JWT,
    db_pool: web::Data<DbPool>
) -> Result<HttpResponse, ApiError> {
    let conn = db_pool.get()?;
    let claims: Claims = jwt::extract_claims(&jwt.0, &config.jwt_secret).map_err(|_| ApiError::AuthError)?;

    return match db::get_existing_notes(&conn, &data.0, &claims.sub){
        Err(error) => {
            Err(error)
        },
        Ok(existing_ids) => {
            let deleted_ids: Vec<String> = data.0.into_iter()
                .filter(|id| !existing_ids.contains(id)).collect();
            Ok(HttpResponse::Ok().json(deleted_ids))
        }
    }
}