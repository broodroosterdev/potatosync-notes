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
extern crate serde_json;
extern crate validator;
extern crate validator_derive;
extern crate rocket_failure;
extern crate rocket_cors;

use std::path::Path;

use diesel_migrations::*;
use dotenv::dotenv;
use rocket_failure::errors::*;
use rocket_cors::CorsOptions;

mod schema;
mod responders;
mod serde_helper;
mod crud;
mod api;
mod models;
mod schemas;
mod db;
mod auth;
mod charset;

use crate::responders::ApiResponse;
use crate::auth::claims::Token;

/// Route used for catching 401 errors e.g. invalid access token
#[catch(401)]
fn token_error() -> ApiResponse {
    ApiResponse {
        body: "InvalidAuth",
        status: Status::Unauthorized,
    }
}

// Used to specify location of migration files
embed_migrations!("migrations");

/// Route for checking connectivity
#[get("/ping")]
fn ping() -> &'static str { "Pong!" }

/// Route for checking if the user is logged in
#[get("/secure-ping")]
fn secure_ping(_token: Token) -> &'static str { "Pong!" }

fn main() {
// Load .env file
    if Path::new(".env").exists() {
        dotenv().ok();
    }
// Run migration
    embedded_migrations::run_with_output(&db::pool::pg_connection(), &mut std::io::stdout()).unwrap();
// Setup CORS
    let cors = CorsOptions::default().to_cors().unwrap();
    let charset = charset::Charset{};
// Start webserver
    let mut rocket = rocket::ignite()
        .attach(cors)
        //Makes sure json responses have correct charset
        .attach(charset)
        .manage(db::pool::connect())
        .register(catchers![token_error])
        .mount("/", routes![ping,secure_ping]);
    rocket = api::endpoints::fuel(rocket);
    rocket.launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ping;
    use rocket::local::Client;
    use crate::note::model::Note;
    use serde_json::{Map, Value};
    use mocktopus::mocking::{Mockable, MockResult};
    use rocket::http::Header;

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
            "tags": "[]",
            "hide_content": false,
            "lock_note": false,
            "uses_biometrics": false,
            "deleted": false,
            "archived": false,
        });
    }

    fn allow_invalid_jwt() {
        read_token.mock_safe(|_| MockResult::Return(Ok(Token {
            sub: "test".parse().unwrap(),
            role: "user".parse().unwrap(),
            token_type: "jwt".parse().unwrap(),
            iat: 420,
            exp: 420,
        })));
    }

    #[test]
    fn test_ping() {
        assert_eq!("Pong!", ping());
    }

    #[test]
    fn test_secure_ping() {
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
    fn save_note_which_is_good() {
        allow_invalid_jwt();
        dotenv::dotenv().ok();
        note_insert_if_empty.mock_safe(|_, _| MockResult::Return(Ok(1)));
        let rocket = rocket::ignite().mount("/", routes![save_note]).manage(db::connect());
        let client = Client::new(rocket).expect("valid rocket instance");
        let json = good_note().to_string();
        let mut response = client.post("/note")
            .header(Header::new("Authorization", "Bearer test"))
            .body(json)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("NoteAddSuccess".to_string()));
    }

    #[test]
    fn dont_save_note_with_no_note_id() {
        allow_invalid_jwt();
        dotenv::dotenv().ok();
        let mut note = good_note();
        note["note_id"] = Value::Null;
        note_insert_if_empty.mock_safe(|_, _| MockResult::Return(Ok(1)));
        let rocket = rocket::ignite().mount("/", routes![save_note]).manage(db::connect());
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.post("/note")
            .header(Header::new("Authorization", "Bearer test"))
            .body(note.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
        println!("{}", response.body_string().unwrap());
    }
}
