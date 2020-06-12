use std::fmt;
use std::fmt::Display;

use rocket_failure::errors::Status;
use serde::export::Formatter;

use crate::status_response::{ApiResponse, StatusResponse};

#[derive(Serialize, Debug)]
pub struct ApiError{
    pub(crate) description: &'static str,
    pub(crate) code: i32
}

impl ApiError{
    pub(crate) fn to_response(&self) -> ApiResponse {
        return ApiResponse {
            json: serde_json::to_string(self).unwrap(),
            status: Status::BadRequest
        };
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return f.write_fmt(format_args!("{}: {}", self.code, self.description));
    }
}

#[derive(Serialize)]
pub struct ApiSucces{
    pub(crate) description: &'static str,
    pub(crate) code: i32
}
impl ApiSucces{
    pub(crate) fn to_response(&self) -> ApiResponse {
        return ApiResponse {
            json: serde_json::to_string(self).unwrap(),
            status: Status::Ok
        };
    }
}
