use chrono::{DateTime, Local};
use diesel::expression::exists::exists;
use diesel::prelude::*;
use diesel::select;
use rayon::prelude::*;
use serde_derive::*;

use crate::db;
use crate::note::model::{Note, NoteLastUpdated, NoteResponse};
use crate::schema::accounts;
use crate::schema::notes;
use crate::status_response::StatusResponse;

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

pub(crate) fn get_notes_by_account(account_id: i32, note_last_updated: NoteLastUpdated, connection: &PgConnection) -> NoteResponse {
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return NoteResponse {
            message: "UserNotFoundError".parse().unwrap(),
            status: false,
            notes: None,
        };
    };
    let notes_result: Result<Vec<Note>, diesel::result::Error> = notes::dsl::notes.filter(notes::account_id.eq(account_id)).load::<Note>(connection);
    return if notes_result.is_ok() {
        let mut synced_notes = notes_result.unwrap();
        let last_updated = DateTime::parse_from_rfc3339(note_last_updated.last_updated.as_ref()).expect("Could not parse DateTime string");
        let mut updated_notes: Vec<Note> = vec![];
        for note in synced_notes {
            let note_date = DateTime::parse_from_rfc3339(note.last_updated.as_str()).expect("Could not parse DateTime string");
            if !note_date.le(&last_updated) {
                let copied: Note = note.clone();
                updated_notes.push(copied);
            }
        };
        NoteResponse {
            message: "NoteListSuccess".parse().unwrap(),
            status: true,
            notes: Some(updated_notes),
        }
    } else {
        NoteResponse {
            message: notes_result.err().unwrap().to_string(),
            status: false,
            notes: None,
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