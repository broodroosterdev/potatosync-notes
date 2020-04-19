use chrono::{DateTime, Local};
use diesel::expression::dsl::count;
use diesel::expression::exists::exists;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::select;
use rayon::prelude::*;
use rocket_failure::errors::Status;
use serde_derive::*;

use crate::db;
use crate::note::model::{Note, NoteLastUpdated, NoteResponse};
use crate::schema::accounts;
use crate::schema::notes;
use crate::status_response::{ApiResponse, StatusResponse};

///Creates note and if they already exist, it will do nothing
pub(crate) fn create(mut note: Note, connection: &PgConnection) -> ApiResponse {
    let insert_result = diesel::insert_into(notes::table)
        .values(&note)
        .on_conflict((notes::columns::note_id, notes::columns::account_id))
        .do_nothing()
        .execute(connection);
    return if insert_result.is_err() {
        ApiResponse {
            json: StatusResponse::new(insert_result.err().unwrap().to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    } else {
        ApiResponse {
            json: StatusResponse::new("NoteCreationSuccess".parse().unwrap(), true).to_string(),
            status: Status::Ok,
        }
    };
}

///Updates note, it will create a new note if it doesnt exist
pub(crate) fn update(note: Note, account_id: String, connection: &PgConnection) -> ApiResponse {
    let note_exists = notes::dsl::notes.select((notes::note_id))
        .filter(notes::note_id.eq(&note.note_id))
        .filter(notes::account_id.eq(&account_id))
        .first::<String>(connection);
    if note_exists.is_err() {
        return ApiResponse {
            json: StatusResponse::new("NoteDoesntExist".to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    }
    let insert_result = diesel::insert_into(notes::table)
        .values(&note)
        .on_conflict((notes::columns::note_id, notes::columns::account_id))
        .do_update()
        .set(&note)
        .execute(connection);
    return if insert_result.is_err() {
        ApiResponse {
            json: StatusResponse::new(insert_result.err().unwrap().to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    } else {
        ApiResponse {
            json: StatusResponse::new("NoteUpdateSuccess".parse().unwrap(), true).to_string(),
            status: Status::Ok,
        }
    };
}

/// Delete single note in DB
pub(crate) fn delete(note_id: String, account_id: String, connection: &PgConnection) -> ApiResponse {
    let note_exists = notes::dsl::notes.select((notes::note_id))
        .filter(notes::note_id.eq(&note_id))
        .filter(notes::account_id.eq(&account_id))
        .first::<String>(connection);
    return if note_exists.is_ok() {
        let delete_result = diesel::delete(notes::table)
            .filter(notes::note_id.eq(&note_id))
            .filter(notes::account_id.eq(&account_id))
            .execute(connection);
        if delete_result.is_err() {
            return ApiResponse {
                json: StatusResponse::new(delete_result.err().unwrap().to_string(), false).to_string(),
                status: Status::BadRequest,
            };
        }
        ApiResponse {
            json: StatusResponse::new("NoteDeleteSuccess".parse().unwrap(), true).to_string(),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            json: StatusResponse::new("NoteDoesntExist".to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    };
}

/// Get list of notes updated after provided timestamp
pub(crate) fn get_notes_by_account(account_id: String, timestamp_last_updated: String, connection: &PgConnection) -> ApiResponse {
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(&account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return ApiResponse {
            json: StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string(),
            status: Status::BadRequest,
        };
    };
    let notes_result: Result<Vec<Note>, diesel::result::Error> = notes::dsl::notes.filter(notes::account_id.eq(&account_id)).load::<Note>(connection);
    return if notes_result.is_ok() {
        let mut synced_notes = notes_result.unwrap();
        let last_updated = DateTime::parse_from_rfc3339(timestamp_last_updated.as_ref()).expect("Could not parse DateTime string");
        let mut updated_notes: Vec<Note> = vec![];
        for note in synced_notes {
            let note_date = note.last_modify_date;
            if !note_date.le(&last_updated) {
                let copied: Note = note.clone();
                updated_notes.push(copied);
            }
        };
        ApiResponse {
            json: NoteResponse {
                message: "NoteListSuccess".parse().unwrap(),
                status: true,
                notes: updated_notes,
            }.to_string(),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            json: StatusResponse::new(notes_result.err().unwrap().to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    };
}

/// Delete all notes of an user
pub(crate) fn delete_all(account_id: String, connection: &PgConnection) -> ApiResponse {
    let delete_result = diesel::delete(notes::table)
        .filter(notes::account_id.eq(&account_id))
        .execute(connection);
    if delete_result.is_err() {
        return ApiResponse {
            json: StatusResponse::new(delete_result.err().unwrap().to_string(), false).to_string(),
            status: Status::BadRequest,
        };
    }
    return ApiResponse {
        json: StatusResponse::new("NotesDeleteSuccess".parse().unwrap(), true).to_string(),
        status: Status::Ok,
    };
}