use chrono::DateTime;
use diesel::expression::exists::exists;
use diesel::prelude::*;
use diesel::select;
use rocket_failure::errors::Status;

use crate::note::model::{Note, NoteResponse, PatchingNote};
use crate::note::repository::{note_exists, note_insert_if_empty, note_patch_if_exists, note_update_if_exists};
use crate::note::responses::*;
use crate::schema::accounts;
use crate::schema::notes;
use crate::status_response::{ApiResponse, StatusResponse};

///Adds note and if they already exist, it will do nothing
pub(crate) fn add(note: Note, connection: &PgConnection) -> ApiResponse {
    if note_exists(&note.note_id, &note.account_id, connection) {
        return ApiResponse {
            json: StatusResponse::new(NOTE_EXISTS.to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    }
    return match note_insert_if_empty(note, connection) {
        Err(error) => ApiResponse {
            json: StatusResponse::new(error, false).to_string(),
            status: Status::BadRequest,
        },
        Ok(changed) => ApiResponse {
            json: StatusResponse::new(NOTE_ADD_SUCCESS.to_string(), true).to_string(),
            status: Status::Ok,
        }
    };
}

///Updates note, it will create a new note if it doesnt exist
pub(crate) fn update(note: Note, account_id: String, connection: &PgConnection) -> ApiResponse {
    if !note_exists(&account_id, &note.note_id, connection) {
        return ApiResponse {
            json: StatusResponse::new(NOTE_NOT_EXISTS.to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    }
    return match note_update_if_exists(note, connection) {
        Err(error) => ApiResponse {
            json: StatusResponse::new(error, false).to_string(),
            status: Status::BadRequest,
        },
        Ok(changed) => ApiResponse {
            json: StatusResponse::new(NOTE_UPDATE_SUCCESS.to_string(), true).to_string(),
            status: Status::Ok,
        }
    }
}

/// Patches note in database. It will only change the fields provided
pub(crate) fn patch(note: PatchingNote, note_id: String, account_id: String, connection: &PgConnection) -> ApiResponse {
    if note.last_modify_date.is_none() {
        return ApiResponse {
            json: StatusResponse::new(NOTE_MISSING_LAST_MODIFY.to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    }
    if !note_exists(&account_id, &note_id, connection) {
        return ApiResponse {
            json: StatusResponse::new(NOTE_NOT_EXISTS.to_string(), false).to_string(),
            status: Status::BadRequest,
        }
    }
    return match note_patch_if_exists(account_id, note_id, note, connection) {
        Err(error) => ApiResponse {
            json: StatusResponse::new(error, false).to_string(),
            status: Status::BadRequest,
        },
        Ok(changed) => ApiResponse {
            json: StatusResponse::new(NOTE_PATCH_SUCCESS.to_string(), true).to_string(),
            status: Status::Ok,
        }
    }
}

/// Delete single note in DB
pub(crate) fn delete(note_id: String, account_id: String, connection: &PgConnection) -> ApiResponse {
    let note_exists = notes::dsl::notes.select(notes::note_id)
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
        let synced_notes = notes_result.unwrap();
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


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

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
}