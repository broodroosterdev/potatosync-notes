

#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;
extern crate lazy_static;
extern crate openssl;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate validator;
extern crate validator_derive;
#[macro_use]
extern crate rocket_failure;


use std::path::Path;

use diesel_migrations::*;
use dotenv::dotenv;
use rocket_contrib::json::Json;
use rocket_failure::errors::*;

mod note;
mod schema;
mod db;
mod responders;
mod responses;
mod status_response;
mod token;

use crate::token::Token;
use crate::note::controller::*;
use crate::note::model::{PatchingNote, SavingNote};
use crate::responders::{ApiError, ApiSuccess};
use crate::responders::ApiResponse;
use crate::status_response::StatusResponse;
use crate::responses::JSON_INVALID;
use serde::de::value::Error;


/// Route for saving note
#[post("/api/notes", data = "<json_note>")]
fn save_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> ApiResponse {
    let note = json_note.0.to_note(token.sub.clone());
    note::controller::add(note, &connection)
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
    delete(note_id, token.sub, &connection)
}

/// Route for deleting all notes of an user
#[delete("/api/notes/all")]
fn delete_all_notes(token: Token, connection: db::Connection) -> ApiResponse {
    delete_all(token.sub, &connection)
}

/// Route for getting all the notes which are updated after provided timestamp
#[get("/api/notes/list?<last_updated>")]
fn get_notes(last_updated: i64, token: Token, connection: db::Connection) -> Result<NoteResponse, ApiResponse> {
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
        body: "InvalidAuth",
        status: Status::Unauthorized,
    }
}

#[catch(422)]
fn invalid_json() -> ApiResponse {
    ApiResponse {
        body: JSON_INVALID.description,
        status: Status::UnprocessableEntity
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
        .register(catchers![token_error, invalid_json])
        .launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ping;
    use rocket::local::Client;
    use crate::note::model::Note;
    use serde_json::{Map, Value};
    use crate::token::read_token;
    use mocktopus::mocking::{Mockable, MockResult};
    use rocket::http::Header;
    use crate::note::repository::note_insert_if_empty;

    fn good_note() -> Value {
        return json!({
            "note_id": "test",
            "title": "test",
            "content": "test",
            "style_json": "{}",
            "starred": false,
            "creation_date": 128,
            "last_modify_date": 200,
            "color": 0,
            "images": "{}",
            "list": false,
            "list_content": "[]",
            "reminders": "[]",
            "hide_content": false,
            "lock_note": false,
            "uses_biometrics": false,
            "deleted": false,
            "archived": false,
        });
    }

    fn allow_invalid_jwt(){
        read_token.mock_safe(|_| MockResult::Return(Ok(Token {
            sub: "test".parse().unwrap(),
            role: "user".parse().unwrap(),
            token_type: "jwt".parse().unwrap(),
            iat: 420,
            exp: 420
        })));
    }
    #[test]
    fn test_ping(){
        assert_eq!("Pong!", ping());
    }

    #[test]
    fn test_secure_ping(){
        allow_invalid_jwt();
        dotenv::dotenv().ok();
        let rocket = rocket::ignite().mount("/", routes![secure_ping]).manage(db::connect());
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/secure-ping")
            .header(Header::new("Authorization", "Bearer test"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Pong!".to_string()));
    }

    #[test]
    fn save_note_which_is_good(){
        allow_invalid_jwt();
        dotenv::dotenv().ok();
        note_insert_if_empty.mock_safe(|_,_| MockResult::Return(Ok(1)));
        let rocket = rocket::ignite().mount("/", routes![save_note]).manage(db::connect());
        let client = Client::new(rocket).expect("valid rocket instance");
        let json = good_note().to_string();
        let mut response = client.post("/api/notes")
            .header(Header::new("Authorization", "Bearer test"))
            .body(json)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("NoteAddSuccess".to_string()));
    }

    #[test]
    fn dont_save_note_with_no_note_id(){
        allow_invalid_jwt();
        dotenv::dotenv().ok();
        let mut note = good_note();
        note["note_id"] = Value::Null;
        note_insert_if_empty.mock_safe(|_,_| MockResult::Return(Ok(1)));
        let rocket = rocket::ignite().mount("/", routes![save_note]).manage(db::connect()).register(catchers!(invalid_json));
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.post("/api/notes")
            .header(Header::new("Authorization", "Bearer test"))
            .body(note.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        println!("{}", response.body_string().unwrap());
    }
}
