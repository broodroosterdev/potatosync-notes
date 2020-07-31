use crate::responders::{ApiResponse};
use rocket::http::Status;

pub(crate) const INTERNAL_DB_ERROR: ApiResponse = ApiResponse {
    body: "InternalDatabaseError",
    status: Status::InternalServerError
};