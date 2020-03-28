#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate rocket_okapi;
extern crate rand;
use std::env;
use std::io::stdout;

use diesel::{Connection, PgConnection};
use diesel_migrations::*;
use dotenv::dotenv;
use rocket::{Data, Request, Response};
use rocket::request::FromRequest;
use rocket::response::{content, status, self, Responder};
use rocket_contrib::json::{Json, JsonValue};
//use token::token;
use rocket_failure::errors::*;
use serde_json::Value;
use rocket::http::ContentType;

use crate::account::controller::{create, login, get_info};
use crate::account::model::{LoginCredentials, NewAccount, Password, Username};
use crate::account::token::{RefreshTokenJson, Token, read_refresh_token, TokenResponse};
use crate::note::controller::*;
use crate::note::model::{Note, NoteId, SavingNote, NoteResponse};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use crate::status_response::StatusResponse;
use crate::status_response::ApiResponse;

mod note;
mod schema;
mod account;
mod db;
mod status_response;

#[openapi]
#[post("/api/users/new", data = "<json_creds>")]
fn new_user(json_creds: Json<NewAccount>, connection: db::Connection) -> ApiResponse {
    create(json_creds.0, &connection)
}

/*#[post("/api/tokens/delete", data = "<token>")]
fn delete_token(token: Data, token: token) {

}*/

#[openapi]
#[post("/api/users/login", data = "<json_credentials>")]
fn login_user(json_credentials: Json<LoginCredentials>, connection: db::Connection) -> ApiResponse {
    let mut response = Response::build();
    let login_result = login(json_credentials.0.clone(), &connection);
    if login_result.is_err() {
        ApiResponse{
            json: login_result.err().unwrap().to_string(),
            status: Status::BadRequest
        }
    } else {
        ApiResponse {
            json: login_result.ok().unwrap().to_string(),
            status: Status::Ok
        }
    }
}

/*#[openapi]
#[post("/api/users/refresh", data = "<json_token>")]
fn refresh_token(json_token: Json<RefreshTokenJson>, connection: db::Connection) -> Json<TokenResponse> {
    let refresh_token = read_refresh_token(json_token.token.as_str());
    return if refresh_token.is_ok() {
        Json(TokenResponse {
            message: "RefreshSuccess".parse().unwrap(),
            status: true,
            token: Some(Token::create_access_token(refresh_token.unwrap().sub.parse().unwrap()))
        })
    } else {
        Json(TokenResponse {
            message: refresh_token.err().unwrap(),
            status: false,
            token: None
        })
    }
}*/

#[openapi]
#[post("/api/users/manage/password", data = "<json_password>")]
fn change_password(json_password: Json<Password>, token: Token, connection: db::Connection) -> content::Json<String> {
    content::Json(account::controller::change_password(token.sub.parse().unwrap(), json_password.0, &connection))
}
/*
#[post("/api/users/manage/username", data = "<json_username>")]
fn change_username(json_username: Json<Username>, token: token, connection: db::Connection){

}

#[post("api/users/manage/image", data = "<json_image>")]
fn change_image(json_image: Json<Image>, token: Token, connection: db::Connection) -> content::Json<String> {

}
*/
#[openapi]
#[get("/api/users/info")]
fn get_user_info(token: Token, connection: db::Connection) -> content::Json<String>{
    content::Json(get_info(token.sub.parse().unwrap(), &connection))
}
#[openapi]
#[post("/api/notes/save", data = "<json_note>")]
fn save_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> content::Json<String> {
    let note = json_note.0.to_note(token.sub.parse().unwrap());
    let create_result = create_or_update(note, &connection);
    content::Json(create_result.to_string())
}
#[openapi]
#[post("/api/notes/delete", data = "<json_note_id>")]
fn delete_note(json_note_id: Json<NoteId>, token: Token, connection: db::Connection) -> Json<StatusResponse> {
    let note_id = json_note_id.0.note_id;
    let delete_result = delete(note_id, token.sub.parse().unwrap(), &connection);
    Json(delete_result)
}
#[openapi]
#[post("/api/notes/deleteall")]
fn delete_all_notes(token: Token, connection: db::Connection) -> Json<StatusResponse> {
    let delete_result = delete_all(token.sub.parse().unwrap(), &connection);
    Json(delete_result)
}
#[openapi]
#[get("/api/notes/list")]
fn get_all_notes(token: Token, connection: db::Connection) -> Json<NoteResponse> {
    let notes = get_notes_by_account(token.sub.parse().unwrap(), &connection);
    Json(notes)
}

#[catch(401)]
fn token_error(req: &Request) -> ApiResponse {
    ApiResponse{
        json: StatusResponse::new("Invalid auth token".parse().unwrap(), false).to_string(),
        status: Status::Unauthorized
    }
}

fn missing_token(error: String) -> content::Json<String> {
    let response = StatusResponse::new(error.parse().unwrap(), false);
    content::Json(response.to_string())
}

embed_migrations!();
fn main() {
    dotenv().ok();
    embedded_migrations::run_with_output(&db::connect().get().unwrap(), &mut stdout());
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes_with_openapi![save_note, get_all_notes, delete_note, delete_all_notes, new_user, login_user])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: Some("../openapi.json".to_owned()),
                urls: None,
            }),
        )
        .register(catchers![token_error])
        .launch();
}
