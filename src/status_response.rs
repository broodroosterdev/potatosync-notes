use okapi::openapi3::RefOr::Ref;
use okapi::openapi3::Responses;
use rocket::{Request, Response, response};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket_contrib::json::JsonValue;
use rocket_failure::errors::Status;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::OpenApiError;
use rocket_okapi::response::OpenApiResponder;
use rocket_okapi::Result;
use rocket_okapi::util::add_schema_response;

use crate::account::model::TokenResponse;

/// Sent back on every request to indicate status and in case it has failed an error of the request
#[derive(Serialize, JsonSchema)]
pub struct StatusResponse {
    pub(crate) message: String,
    pub(crate) status: bool,
}

impl StatusResponse {
    pub fn new(message: String, status: bool) -> StatusResponse {
        StatusResponse {
            message,
            status,
        }
    }
}

impl ToString for StatusResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// Struct used for sending the StatusResponse and other json back with a specific http code
#[derive(Debug)]
pub(crate) struct ApiResponse {
    /// The json to be sent back in serialized form
    pub(crate) json: String,
    /// The statuscode to be used for the response (Status::Ok,Status::NotFound etc.)
    pub(crate) status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}