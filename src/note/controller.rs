use diesel::prelude::*;
use serde_derive::*;

use crate::db;
use crate::note::model::{Note, NoteResponse};
use crate::status_response::StatusResponse;
use crate::schema::notes;
use crate::schema::accounts;
use diesel::select;
use diesel::expression::exists::exists;

pub(crate) fn create_or_update(note: Note, connection: &PgConnection) -> StatusResponse {
    let insert_result = diesel::insert_into(notes::table)
        .values(&note)
        .on_conflict((notes::columns::note_id, notes::columns::account_id))
        .do_update()
        .set(&note)
        .execute(connection);
    return if insert_result.is_err() {
        StatusResponse::new(insert_result.err().unwrap().to_string(), false)
    } else {
        StatusResponse::new("NoteCreationSuccess".parse().unwrap(), true)
    };
}

pub(crate) fn get_by_id(id: i32, connection: &PgConnection) -> Note {
    notes::dsl::notes.filter(notes::note_id.eq(id)).first::<Note>(connection).unwrap()
}

pub(crate) fn delete(note_id: i32, account_id: i32, connection: &PgConnection) -> StatusResponse {
    let note_exists = notes::dsl::notes.select((notes::note_id))
        .filter(notes::note_id.eq(&note_id))
        .filter(notes::account_id.eq(&account_id))
        .first::<i32>(connection);
    return if note_exists.is_ok() {
        let delete_result = diesel::delete(notes::table)
            .filter(notes::note_id.eq(&note_id))
            .filter(notes::account_id.eq(&account_id))
            .execute(connection);
        if delete_result.is_err() {
            return StatusResponse::new(delete_result.err().unwrap().to_string(), false);
        }
        StatusResponse::new("NoteDeleteSuccess".parse().unwrap(), true)
    } else {
        StatusResponse::new(note_exists.err().unwrap().to_string(), false)
    };
}

pub(crate) fn get_notes_by_account(account_id: i32, connection: &PgConnection) -> NoteResponse {
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return NoteResponse{
            message: "UserNotFoundError".parse().unwrap(),
            status: false,
            notes: None
        };
    };
    let notes = notes::dsl::notes.filter(notes::account_id.eq(account_id)).load::<Note>(connection);
    return if notes.is_ok() {
        NoteResponse {
            message: "NoteListSuccess".parse().unwrap(),
            status: true,
            notes: Some(notes.unwrap())
        }
    } else {
        NoteResponse {
            message: notes.err().unwrap().to_string(),
            status: false,
            notes: None
        }
    };
}

pub(crate) fn delete_all(account_id: i32, connection: &PgConnection) -> StatusResponse {
    let delete_result = diesel::delete(notes::table)
        .filter(notes::account_id.eq(&account_id))
        .execute(connection);
    if delete_result.is_err() {
        return StatusResponse::new(delete_result.err().unwrap().to_string(), false);
    }
    StatusResponse::new("NotesDeleteSuccess".parse().unwrap(), true)
}