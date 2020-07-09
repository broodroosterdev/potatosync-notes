#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate lazy_static;
extern crate openssl;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate validator;
extern crate validator_derive;

use std::path::Path;

use diesel_migrations::*;
use dotenv::dotenv;
use rocket_contrib::json::Json;
use rocket_failure::errors::*;

mod note;
mod schema;
mod db;
mod responders;
mod status_response;
mod token;

use crate::token::Token;
use crate::note::controller::*;
use crate::note::model::{PatchingNote, SavingNote};
use crate::responders::{ApiError, ApiSuccess};
use crate::responders::ApiResponse;
use crate::status_response::StatusResponse;


/// Route for saving note
#[post("/api/notes", data = "<json_note>")]
fn save_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> Result<ApiSuccess, ApiError> {
    let note = json_note.0.to_note(token.sub.clone());
    note::controller::add(note, &connection)
}

/// Route for updating note
#[put("/api/notes", data = "<json_note>")]
fn update_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> Result<ApiSuccess, ApiError> {
    let note = json_note.to_note(token.sub.clone());
    note::controller::update(note, token.sub, &connection)
}

#[patch("/api/notes/<note_id>", data = "<json_note>")]
fn patch_note(note_id: String, json_note: Json<PatchingNote>, token: Token, connection: db::Connection) -> Result<ApiSuccess, ApiError> {
    let note = json_note.0;
    note::controller::patch(note, note_id, token.sub, &connection)
}

/// Route for deleting single note identified by id
#[delete("/api/notes/<note_id>")]
fn delete_note(note_id: String, token: Token, connection: db::Connection) -> Result<ApiSuccess, ApiError> {
    delete(note_id, token.sub.parse().unwrap(), &connection)
}

/// Route for deleting all notes of an user
#[delete("/api/notes/all")]
fn delete_all_notes(token: Token, connection: db::Connection) -> Result<ApiSuccess, ApiError> {
    delete_all(token.sub.parse().unwrap(), &connection)
}

/// Route for getting all the notes which are updated after provided timestamp
#[get("/api/notes/list?<last_updated>")]
fn get_notes(last_updated: i64, token: Token, connection: db::Connection) -> Result<NoteResponse, ApiError> {
    get_notes_by_account(token.sub, last_updated, &connection)
}

/// Route for checking connectivity
#[get("/ping")]
fn ping() -> String {
    return "Pong!".parse().unwrap();
}

#[get("/secure-ping")]
fn secure_ping(_token: Token) -> String {return "Pong!".parse().unwrap();}

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
        .mount("/", routes![save_note, update_note, patch_note, delete_note, delete_all_notes, get_notes])
        .mount("/", routes![ping, secure_ping])
        .register(catchers![token_error])
        .launch();
}
