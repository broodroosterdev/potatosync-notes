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
use crate::account::model::{Image, LoginCredentials, NewAccount, Password, PatchingAccount, Username};
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

/// Route for logging in user
#[post("/api/users/login", data = "<json_credentials>")]
fn login_user(json_credentials: Json<LoginCredentials>, connection: db::Connection) -> ApiResponse {
    let login_result = login(json_credentials.0.clone(), &connection);
    if login_result.is_err() {
        ApiResponse {
            json: login_result.err().unwrap().to_string(),
            status: Status::BadRequest,
        }
    } else {
        ApiResponse {
            json: login_result.ok().unwrap().to_string(),
            status: Status::Ok,
        }
    }
}

/// Route for logging out and deleting refresh_token to disable refreshing
/*#[post("/api/tokens/delete", data = "<token>")]
fn delete_token(token: Data, token: token) {

}*/

/// Route for refreshing access_token with refresh_token
#[post("/api/users/refresh", data = "<json_token>")]
fn refresh_user(json_token: Json<RefreshTokenJson>, connection: db::Connection) -> ApiResponse {
    let token = read_refresh_token(json_token.token.as_str());
    if token.is_err() {
        return ApiResponse {
            json: StatusResponse::new(token.err().unwrap(), false).to_string(),
            status: Status::BadRequest,
        };
    }
    refresh_token(token.unwrap(), &connection)
}

/// Route for changing password of user
#[patch("/api/users/info", data = "<json_patch>")]
fn change_user_info(json_patch: Json<PatchingAccount>, token: Token, connection: db::Connection) -> ApiResponse {
    account::controller::change_info(token.sub.parse().unwrap(), json_patch.0, &connection)
}

/// Route for getting user info
#[get("/api/users/info")]
fn get_user_info(token: Token, connection: db::Connection) -> content::Json<String> {
    content::Json(get_info(token.sub.parse().unwrap(), &connection))
}


/// Route for saving note
#[post("/api/notes", data = "<json_note>")]
fn save_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> ApiResponse {
    let note = json_note.0.to_note(token.sub.parse().unwrap());
    note::controller::create(note, &connection)
}

/// Route for updating note
#[put("/api/notes", data = "<json_note>")]
fn update_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> ApiResponse {
    let note = json_note.0.to_note(token.sub.parse().unwrap());
    note::controller::create(note, &connection)
}

/// Route for deleting single note identified by id
#[delete("/api/notes/<note_id>")]
fn delete_note(note_id: i32, token: Token, connection: db::Connection) -> ApiResponse {
    delete(note_id, token.sub.parse().unwrap(), &connection)
}

/// Route for deleting all notes of an user
#[delete("/api/notes/all")]
fn delete_all_notes(token: Token, connection: db::Connection) -> ApiResponse {
    delete_all(token.sub.parse().unwrap(), &connection)
}

/// Route for getting all the notes which are updated after provided timestamp
#[get("/api/notes/list?<last_updated>")]
fn get_notes(last_updated: String, token: Token, connection: db::Connection) -> ApiResponse {
    get_notes_by_account(token.sub.parse().unwrap(), last_updated, &connection)
}


/// Route used for catching 401 errors e.g. invalid access token
#[catch(401)]
fn token_error(req: &Request) -> ApiResponse {
    ApiResponse {
        json: StatusResponse::new("Invalid auth token".parse().unwrap(), false).to_string(),
        status: Status::Unauthorized,
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
        .mount("/", routes![new_user, verify_user, login_user, refresh_user, change_user_info, get_user_info])
        .mount("/", routes![save_note, update_note, delete_note, delete_all_notes, get_notes])
        .register(catchers![token_error])
        .launch();
}
