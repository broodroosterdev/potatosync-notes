#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate openssl;
extern crate rand;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tera;
extern crate validator;
extern crate validator_derive;

use std::path::Path;

use diesel_migrations::*;
use dotenv::dotenv;
use rocket::response::{content, Redirect};
use rocket_contrib::json::Json;
use rocket_failure::errors::*;

use crate::account::controller::{create, get_info, login, verify_email};
use crate::account::model::{LoginCredentials, NewAccount, PatchingAccount};
use crate::account::token::{read_refresh_token, refresh_token, RefreshTokenJson, Token};
use crate::note::controller::*;
use crate::note::model::{PatchingNote, SavingNote};
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
fn verify_user(id: String, token: String, connection: db::Connection) -> Redirect {
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

/// Route for deleting account and notes
#[delete("/api/users", data = "<json_token>")]
fn delete_user(json_token: Json<RefreshTokenJson>, _token: Token, connection: db::Connection) -> ApiResponse {
    let token = read_refresh_token(json_token.token.as_str());
    if token.is_err() {
        return ApiResponse {
            json: StatusResponse::new(token.err().unwrap(), false).to_string(),
            status: Status::BadRequest,
        };
    }
    account::controller::delete_user(token.unwrap(), &connection)
}

/// Route for saving note
#[post("/api/notes", data = "<json_note>")]
fn save_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> ApiResponse {
    let note = json_note.0.to_note(token.sub.clone());
    note::controller::create(note, &connection)
}

/// Route for updating note
#[put("/api/notes", data = "<json_note>")]
fn update_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> ApiResponse {
    let note = json_note.to_note(token.sub.clone());
    note::controller::update(note, token.sub, &connection)
}

#[patch("/api/notes/<note_id>", data = "<json_note>")]
fn patch_note(note_id: String, json_note: Json<PatchingNote>, token: Token, connection: db::Connection) -> ApiResponse {
    let note = json_note.0;
    note::controller::patch(note, note_id, token.sub, &connection)
}

/// Route for deleting single note identified by id
#[delete("/api/notes/<note_id>")]
fn delete_note(note_id: String, token: Token, connection: db::Connection) -> ApiResponse {
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
fn token_error() -> ApiResponse {
    ApiResponse {
        json: StatusResponse::new("Invalid auth token".parse().unwrap(), false).to_string(),
        status: Status::Unauthorized,
    }
}

// Used to specify location of migration files
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
        .mount("/", routes![new_user, verify_user, login_user, refresh_user, change_user_info, get_user_info, delete_user])
        .mount("/", routes![save_note, update_note, patch_note, delete_note, delete_all_notes, get_notes])
        .register(catchers![token_error])
        .launch();
}
