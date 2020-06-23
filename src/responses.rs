use std::fmt;
use std::fmt::Display;

use rocket::{Request, response, Response};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket_failure::errors::Status;
use serde::export::Formatter;

use crate::account::model::TokenAccount;
use crate::account::responses::LOGIN_SUCCESS;
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

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(serde_json::to_string(&self).unwrap().respond_to(&req).unwrap())
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .ok()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return f.write_fmt(format_args!("{}: {}", self.code, self.description));
    }
}

#[derive(Serialize, Debug)]
pub struct ApiSuccess {
    pub(crate) description: &'static str,
    pub(crate) code: i32
}

impl ApiSuccess {
    pub(crate) fn to_response(&self) -> ApiResponse {
        return ApiResponse {
            json: serde_json::to_string(self).unwrap(),
            status: Status::Ok
        };
    }
}

impl<'r> Responder<'r> for ApiSuccess {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(serde_json::to_string(&self).unwrap().respond_to(&req).unwrap())
            .status(Status::Ok)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Serialize, Debug)]
pub struct TokenSuccess {
    pub(crate) description: &'static str,
    pub(crate) code: i32,
    pub(crate) access_token: &'static str,
    pub(crate) refresh_token: &'static str,
    pub(crate) id_token: &'static str,
}

impl TokenSuccess {
    pub(crate) fn new(access_token: &'static str, refresh_token: &'static str, id_token: &'static str) -> Self {
        return TokenSuccess{
            description: LOGIN_SUCCESS.description,
            code: LOGIN_SUCCESS.code,
            access_token,
            refresh_token,
            id_token
        }
    }
}

impl<'r> Responder<'r> for TokenSuccess {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(serde_json::to_string(&self).unwrap().respond_to(&req).unwrap())
            .status(Status::Ok)
            .header(ContentType::JSON)
            .ok()
    }
}

impl ToString for TokenSuccess {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
