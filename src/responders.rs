use std::fmt;
use std::fmt::Display;

use rocket::{Request, response, Response};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::http::Status;
use serde::export::Formatter;


/// Struct used for sending the StatusResponse and other json back with a specific http code
#[derive(Debug)]
pub(crate) struct ApiResponse {
    /// The body to be sent back
    pub(crate) body: &'static str,
    /// The statuscode to be used for the response (Status::Ok,Status::NotFound etc.)
    pub(crate) status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.body.respond_to(&req).unwrap())
            .status(self.status)
            .ok()
    }
}

#[derive(Serialize, Debug)]
pub struct ApiError{
    pub(crate) description: &'static str,
    pub(crate) code: i32
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

impl<'r> Responder<'r> for ApiSuccess {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(serde_json::to_string(&self).unwrap().respond_to(&req).unwrap())
            .status(Status::Ok)
            .header(ContentType::JSON)
            .ok()
    }
}