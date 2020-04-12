#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate openssl;
extern crate rand;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tera;
extern crate validator;
#[macro_use]
extern crate validator_derive;


use std::env;
use std::io::{stdin, stdout};
use std::path::Path;

use diesel::{Connection, PgConnection};
use diesel_migrations::*;
use dotenv::dotenv;
use rocket::{Data, Request, Response};
use rocket::http::ContentType;
use rocket::request::FromRequest;
use rocket::response::{self, content, Redirect, Responder, status};
use rocket_contrib::json::{Json, JsonValue};
use rocket_failure::errors::*;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use serde_json::Value;

use crate::account::controller::{create, get_info, login, verify_email};
use crate::account::model::{LoginCredentials, NewAccount, Password, Username};
use crate::account::token::{read_refresh_token, refresh_token, RefreshTokenJson, Token};
use crate::note::controller::*;
use crate::note::model::{Note, NoteId, NoteLastUpdated, NoteResponse, SavingNote};
use crate::status_response::ApiResponse;
use crate::status_response::StatusResponse;

mod note;
mod schema;
mod account;
mod db;
mod status_response;


/// Route for registering user
#[post("/api/users/new", data = "<json_creds>")]
fn new_user(json_creds: Json<NewAccount>, connection: db::Connection) -> ApiResponse {
    create(json_creds.0, &connection)
}

/// Route for verifying email token
#[get("/api/users/verify/<id>/<token>")]
fn verify_user(id: i32, token: String, connection: db::Connection) -> Redirect {
    let verify_result = verify_email(id, token, &connection);
    Redirect::to(format!("/api/users/verify?successful={}&message={}", verify_result.status, verify_result.message))
}

/// Route for logging out and deleting refresh_token to disable refreshing
/*#[post("/api/tokens/delete", data = "<token>")]
fn delete_token(token: Data, token: token) {

}*/

/// Route for logging in user
#[post("/api/users/login", data = "<json_credentials>")]
fn login_user(json_credentials: Json<LoginCredentials>, connection: db::Connection) -> ApiResponse {
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

/// Route for refreshing access_token with refresh_token
#[post("/api/users/refresh", data = "<json_token>")]
fn refresh_user(json_token: Json<RefreshTokenJson>, connection: db::Connection) -> ApiResponse {
    let token = read_refresh_token(json_token.token.as_str());
    if token.is_err() {
        return ApiResponse {
            json: StatusResponse::new(token.err().unwrap(), false).to_string(),
            status: Status::BadRequest,
        }
    }
    refresh_token(token.unwrap(), &connection)
}

/// Route for changing password of user
#[post("/api/users/manage/password", data = "<json_password>")]
fn change_password(json_password: Json<Password>, token: Token, connection: db::Connection) -> content::Json<String> {
    content::Json(account::controller::change_password(token.sub.parse().unwrap(), json_password.0, &connection))
}

/// Route for changing username of user
/*
#[post("/api/users/manage/username", data = "<json_username>")]
fn change_username(json_username: Json<Username>, token: token, connection: db::Connection){

}

/// Route for changing profile picture of user
#[post("api/users/manage/image", data = "<json_image>")]
fn change_image(json_image: Json<Image>, token: Token, connection: db::Connection) -> content::Json<String> {

}
*/

/// Route for getting user info
#[get("/api/users/info")]
fn get_user_info(token: Token, connection: db::Connection) -> content::Json<String> {
    content::Json(get_info(token.sub.parse().unwrap(), &connection))
}

/// Route for saving note
#[post("/api/notes/save", data = "<json_note>")]
fn save_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> content::Json<String> {
    let note = json_note.0.to_note(token.sub.parse().unwrap());
    let create_result = create_or_update(note, &connection);
    content::Json(create_result.to_string())
}

/// Route for deleting single note identified by id
#[post("/api/notes/delete", data = "<json_note_id>")]
fn delete_note(json_note_id: Json<NoteId>, token: Token, connection: db::Connection) -> Json<StatusResponse> {
    let note_id = json_note_id.0.note_id;
    let delete_result = delete(note_id, token.sub.parse().unwrap(), &connection);
    Json(delete_result)
}

/// Route for deleting all notes of an user
#[post("/api/notes/deleteall")]
fn delete_all_notes(token: Token, connection: db::Connection) -> Json<StatusResponse> {
    let delete_result = delete_all(token.sub.parse().unwrap(), &connection);
    Json(delete_result)
}

/// Route for getting all the notes which are updated after provided timestamp
#[get("/api/notes/list", data = "<json_last_updated>")]
fn get_all_notes(json_last_updated: Json<NoteLastUpdated>, token: Token, connection: db::Connection) -> Json<NoteResponse> {
    let notes = get_notes_by_account(token.sub.parse().unwrap(), json_last_updated.0, &connection);
    Json(notes)
}

/// Route used for catching 401 errors e.g. invalid access token
#[catch(401)]
fn token_error(req: &Request) -> ApiResponse {
    ApiResponse{
        json: StatusResponse::new("Invalid auth token".parse().unwrap(), false).to_string(),
        status: Status::Unauthorized
    }
}

/// Used to specify location of migration files
embed_migrations!("migrations");
fn main() {
    // Load .env file
    if Path::new(".env").exists() {
        dotenv().ok();
    }
    // Run migration
    embedded_migrations::run_with_output(&db::connect().get().unwrap(), &mut std::io::stdout()).unwrap();
    // Start webserver
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![save_note, get_all_notes, delete_note, delete_all_notes, new_user, login_user, change_password, refresh_user])
        .mount("/", routes![verify_user])
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
