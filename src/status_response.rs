use rocket::response::Responder;
use rocket::{Response, Request, response};
use rocket_failure::errors::Status;
use rocket::http::ContentType;
use rocket_contrib::json::JsonValue;
use rocket_okapi::response::OpenApiResponder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::OpenApiError;
use okapi::openapi3::Responses;
use rocket_okapi::Result;
use okapi::openapi3::RefOr::Ref;
use rocket_okapi::util::add_schema_response;
use crate::account::model::TokenResponse;

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

#[derive(Debug)]
pub(crate) struct ApiResponse {
    pub(crate) json: String,
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

impl OpenApiResponder<'_> for ApiResponse {
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses> {
        let mut responses = Responses::default();
        let token_schema = gen.json_schema::<TokenResponse>();
        add_schema_response(&mut responses, 200, "application/json", token_schema)?;
        let status_schema = gen.json_schema::<StatusResponse>();
        add_schema_response(&mut responses, 400, "application/json", status_schema)?;
        Ok(responses)
    }
}
