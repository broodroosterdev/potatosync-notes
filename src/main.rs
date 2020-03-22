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

use std::env;
use std::io::stdout;

use diesel::{Connection, PgConnection};
use diesel_migrations::*;
use dotenv::dotenv;
use rocket::{Data, Request};
use rocket::request::FromRequest;
use rocket::response::content;
use rocket_contrib::json::{Json, JsonValue};
//use token::token;
use rocket_failure::errors::*;
use serde_json::Value;

use crate::account::controller::{create, login};
use crate::account::model::{LoginCredentials, NewAccount};
use crate::account::token::{RefreshTokenJson, Token};
use crate::note::controller::*;
use crate::note::model::{Note, NoteId, SavingNote};
use crate::response::StatusResponse;

mod note;
mod schema;
mod account;
mod db;
mod response;

#[post("/api/users/new", data = "<json_creds>")]
fn new_user(json_creds: Json<NewAccount>, connection: db::Connection) -> content::Json<String> {
    println!("Recieved request");
    content::Json(create(json_creds.0, &connection))
}

/*#[post("/api/tokens/delete", data = "<token>")]
fn delete_token(token: Data, token: token) {

}*/

#[post("/api/users/login", data = "<json_credentials>")]
fn login_user(json_credentials: Json<LoginCredentials>, connection: db::Connection) -> content::Json<String> {
    account::controller::login(json_credentials.0.clone(), &connection);
    content::Json(login(json_credentials.0.clone(), &connection))
}

#[post("/api/users/refresh", data = "<json_token>")]
fn refresh_token(json_token: Json<RefreshTokenJson>, connection: db::Connection) -> content::Json<String> {
    content::Json(Token::create_access_token(json_token.0.token.sub.parse().unwrap()))
}
/*
#[post("/api/tokens/manage/password", data = "<password>")]
fn change_password(password: Json<Value>, token: token, connection: db::Connection) {

}

#[post("/api/tokens/manage/tokenname", data = "<tokenname>")]
fn change_tokenname(tokenname: Json<Value>, token: token, connection: db::Connection){

}

#[get("/api/tokens/info")]
fn get_token_info(token: token, connection: db::Connection){

}*/

#[post("/api/notes/save", data = "<json_note>")]
fn save_note(json_note: Json<SavingNote>, token: Token, connection: db::Connection) -> content::Json<String> {
    let note = json_note.0.to_note(token.sub.parse().unwrap());
    let create_result = create_or_update(note, &connection);
    content::Json(create_result.to_string())
}

#[post("/api/notes/delete", data = "<json_note_id>")]
fn delete_note(json_note_id: Json<NoteId>, token: Token, connection: db::Connection) -> content::Json<String> {
    let note_id = json_note_id.0.note_id;
    let delete_result = delete(note_id, token.sub.parse().unwrap(), &connection);
    content::Json(delete_result.to_string())
}

#[post("/api/notes/deleteall")]
fn delete_all_notes(token: Token, connection: db::Connection) -> content::Json<String> {
    let delete_result = delete_all(token.sub.parse().unwrap(), &connection);
    content::Json(delete_result.to_string())
}

#[get("/api/notes/list")]
fn get_all_notes(token: Token, connection: db::Connection) -> content::Json<String> {
    let notes = get_notes_by_account(token.sub.parse().unwrap(), &connection);
    return if notes.is_err() {
        content::Json(notes.err().unwrap().to_string())
    } else {
        content::Json(serde_json::to_string(&notes.ok().unwrap()).unwrap())
    };
}


#[catch(403)]
fn token_error(req: &Request) -> content::Json<String> {
    content::Json("Forbidden".parse().unwrap())
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
        .mount("/", routes![save_note, get_all_notes, delete_note, delete_all_notes])
        .mount("/", routes![new_user, login_user])
        .register(catchers![token_error])
        .launch();
}
