use diesel::result::Error as DBError;
use r2d2::Error as PoolError;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};

#[derive(Display, From, Debug)]
pub enum ApiError {
    NotFound,
    Duplicate,
    DBError(DBError),
    PoolError(PoolError),
    AuthError
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Self::NotFound => HttpResponse::NotFound().finish(),
            Self::Duplicate => HttpResponse::BadRequest().body("Duplicate"),
            Self::AuthError => HttpResponse::Unauthorized().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}