use chrono::{TimeZone, Utc};
use diesel::prelude::*;
use rocket_failure::errors::Status;

use crate::note::model::{Note, PatchingNote};
use crate::note::repository::{note_delete, note_delete_all, note_exists, note_insert_if_empty, note_patch_if_exists, note_update_if_exists, notes_get_all};
use crate::note::responses::*;
use crate::responders::{ApiSuccess, ApiError};
use rocket::response::Responder;
use rocket::{Response, Request, response};
use rocket::http::ContentType;


///Adds note and if they already exist, it will do nothing
pub(crate) fn add(note: Note, connection: &PgConnection) -> Result<ApiSuccess, ApiError> {
    if note_exists(&note.account_id, &note.note_id, connection) {
        return Err(NOTE_EXISTS);
    }
    return match note_insert_if_empty(note, connection) {
        Err(error) => {
            println!("Unable to insert note: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(_changed) => Ok(NOTE_ADD_SUCCESS)
    };
}

///Updates note, it will create a new note if it doesnt exist
pub(crate) fn update(note: Note, account_id: String, connection: &PgConnection) -> Result<ApiSuccess, ApiError> {
    if !note_exists(&account_id, &note.note_id, connection) {
        return Err(NOTE_NOT_EXISTS);
    }
    return match note_update_if_exists(note, connection) {
        Err(error) => {
            println!("Unable to update note: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(_changed) => Ok(NOTE_UPDATE_SUCCESS)
    };
}

/// Patches note in database. It will only change the fields provided
pub(crate) fn patch(note: PatchingNote, note_id: String, account_id: String, connection: &PgConnection) -> Result<ApiSuccess, ApiError> {
    if note.last_modify_date.is_none() {
        return Err(NOTE_MISSING_LAST_MODIFY);
    }
    if !note_exists(&account_id, &note_id, connection) {
        return Err(NOTE_NOT_EXISTS);
    }
    return match note_patch_if_exists(account_id, note_id, note, connection) {
        Err(error) => {
            println!("Unable to patch note: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(_changed) => Ok(NOTE_PATCH_SUCCESS)
    };
}

/// Delete single note in DB
pub(crate) fn delete(note_id: String, account_id: String, connection: &PgConnection) -> Result<ApiSuccess, ApiError> {
    if !note_exists(&account_id, &note_id, connection) {
        return Err(NOTE_NOT_EXISTS);
    }
    return match note_delete(account_id, note_id, connection) {
        Err(error) => {
            println!("Unable to delete note: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(_changed) => Ok(NOTE_DELETE_SUCCESS)
    };
}

/// Delete all notes of an user
pub(crate) fn delete_all(account_id: String, connection: &PgConnection) -> Result<ApiSuccess, ApiError> {
    return match note_delete_all(account_id, connection) {
        Err(error) => {
            println!("Unable to delete all notes: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(_) => Ok(NOTES_DELETE_SUCCESS)
    };
}

/// Struct used when server needs to return list of notes along with status of request
#[derive(Serialize, Deserialize)]
pub struct NoteResponse {
    pub(crate) message: &'static str,
    pub(crate) notes: Vec<Note>,
}

impl<'r> Responder<'r> for NoteResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(serde_json::to_string(&self).unwrap().respond_to(&req).unwrap())
            .status(Status::Ok)
            .header(ContentType::JSON)
            .ok()
    }
}

/// Get list of notes updated after provided timestamp
pub(crate) fn get_notes_by_account(account_id: String, timestamp_last_updated: i64, connection: &PgConnection) -> Result<NoteResponse, ApiError> {
    return match notes_get_all(&account_id, connection) {
        Err(error) => {
            println!("Unable to get notes: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(synced_notes) => {
            let last_updated = Utc.timestamp(timestamp_last_updated / 1000, 0);
            let mut updated_notes: Vec<Note> = vec![];
            for note in synced_notes {
                if !note.last_modify_date.le(&last_updated) {
                    updated_notes.push(note);
                }
            }
            Ok(NoteResponse {
                message: NOTE_LIST_SUCCESS.description,
                notes: updated_notes,
            })
        }
    };
}


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::ops::Sub;

    use chrono::{Duration, FixedOffset, TimeZone, Utc};
    use mocktopus::mocking::*;

    use crate::db;
    use crate::note::model::Note;
    use crate::note::repository::{note_exists, note_insert_if_empty};

    use super::*;

    #[test]
    fn success_when_adding_nonexisting() {
        dotenv::dotenv().ok();
        note_exists.mock_safe(|_, _, _| MockResult::Return(false));
        note_insert_if_empty.mock_safe(|_, _| MockResult::Return(Ok(1)));
        let note: Note = Note::mock_empty();
        let result = add(note, &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_ADD_SUCCESS.to_string(), true).to_string(),
            status: Status::Ok,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn error_when_adding_existing() {
        dotenv::dotenv().ok();
        note_exists.mock_safe(|_, _, _| MockResult::Return(true));
        let note: Note = Note::mock_empty();
        let result = add(note, &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_EXISTS.to_string(), false).to_string(),
            status: Status::BadRequest,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn success_when_updating_existing() {
        dotenv::dotenv().ok();
        note_exists.mock_safe(|_, _, _| MockResult::Return(true));
        note_update_if_exists.mock_safe(|_, _| MockResult::Return(Ok(1)));
        let note: Note = Note::mock_empty();
        let result = update(note, "".parse().unwrap(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_UPDATE_SUCCESS.to_string(), true).to_string(),
            status: Status::Ok,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn error_when_updating_nonexistent() {
        dotenv::dotenv().ok();
        note_exists.mock_safe(|_, _, _| MockResult::Return(false));
        let note: Note = Note::mock_empty();
        let result = update(note, "".parse().unwrap(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_NOT_EXISTS.to_string(), false).to_string(),
            status: Status::BadRequest,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn success_when_patching_existing() {
        dotenv::dotenv().ok();
        note_exists.mock_safe(|_, _, _| MockResult::Return(true));
        note_patch_if_exists.mock_safe(|_, _, _, _| MockResult::Return(Ok(1)));
        let note: PatchingNote = PatchingNote::mock_empty_with_last_modify();
        let result = patch(note, "".to_string(), "".to_string(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_PATCH_SUCCESS.to_string(), true).to_string(),
            status: Status::Ok,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn error_when_patching_nonexisting() {
        dotenv::dotenv().ok();
        note_exists.mock_safe(|_, _, _| MockResult::Return(false));
        let note: PatchingNote = PatchingNote::mock_empty_with_last_modify();
        let result = patch(note, "".to_string(), "".to_string(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_NOT_EXISTS.to_string(), false).to_string(),
            status: Status::BadRequest,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn error_when_patching_without_last_modify() {
        dotenv::dotenv().ok();
        let note: PatchingNote = PatchingNote::mock_empty();
        let result = patch(note, "".to_string(), "".to_string(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_MISSING_LAST_MODIFY.to_string(), false).to_string(),
            status: Status::BadRequest,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn success_when_deleting_existing() {
        dotenv::dotenv().ok();
        note_exists.mock_safe(|_, _, _| MockResult::Return(false));
        note_delete.mock_safe(|_, _, _| MockResult::Return(Ok(1)));
        let result = delete("".to_string(), "".to_string(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_NOT_EXISTS.to_string(), false).to_string(),
            status: Status::BadRequest,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn error_when_deleting_nonexistent() {
        dotenv::dotenv().ok();
        note_exists.mock_safe(|_, _, _| MockResult::Return(false));
        let result = delete("".to_string(), "".to_string(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTE_NOT_EXISTS.to_string(), false).to_string(),
            status: Status::BadRequest,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn success_when_deleting_all_success() {
        dotenv::dotenv().ok();
        note_delete_all.mock_safe(|_, _| MockResult::Return(Ok(5)));
        let result = delete_all("".to_string(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new(NOTES_DELETE_SUCCESS.to_string(), true).to_string(),
            status: Status::Ok,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn error_when_deleting_all_error() {
        dotenv::dotenv().ok();
        note_delete_all.mock_safe(|_, _| MockResult::Return(Err("Error".to_string())));
        let result = delete_all("".to_string(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: StatusResponse::new("Error".to_string(), false).to_string(),
            status: Status::BadRequest,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }

    #[test]
    fn success_when_getting_notes_after_modify() {
        dotenv::dotenv().ok();
        let date = Utc::now();
        let mut note = Note::mock_empty();
        note.last_modify_date = date;
        note.creation_date = note.last_modify_date.clone();
        let mut note_copy = note.clone();
        notes_get_all.mock_safe(move |_, _| MockResult::Return({
            Ok(vec![note_copy.clone()])
        }));
        let last_modify = note.last_modify_date.clone() - Duration::minutes(1);
        let result = get_notes_by_account("".to_string(), last_modify.timestamp_millis(), &db::connect().get().unwrap());
        let correct_result = ApiResponse {
            json: NoteResponse {
                message: NOTE_LIST_SUCCESS.to_string(),
                status: true,
                notes: vec![note],
            }.to_string(),
            status: Status::Ok,
        };
        println!("{} : {}", result.status, result.json);
        assert_eq!(result.json, correct_result.json);
        assert_eq!(result.status, correct_result.status);
    }
}