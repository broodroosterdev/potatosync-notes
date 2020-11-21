use rocket::{Rocket, Request, response, Response};
use rocket_contrib::json::Json;
use crate::crud::notes;
use crate::db::guard::Connection;
use crate::models::notes::Note;
use crate::schemas::notes::{SavingNote, PatchingNote, NoteResponse, DeletedResponse};
use crate::auth::claims::Token;
use crate::responders::ApiResponse;
use rocket::http::{Status, ContentType};
use crate::crud::notes::*;
use rocket::response::Responder;
use chrono::{Utc, TimeZone};

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/note", routes![
        create,
        update,
        patch,
        delete,
        delete_all,
        get_updated,
        get_deleted,
    ])
}

/// Route for saving note
#[post("/", data = "<json_note>")]
fn create(json_note: Json<SavingNote>, token: Token, connection: Connection) -> ApiResponse {
    let note = json_note.0.to_note(token.sub.clone());
    if notes::note_exists(&note.account_id, &note.note_id, &*connection) {
        return NOTE_ALREADY_EXISTS;
    }
    return match notes::note_insert_if_empty(note, &*connection) {
        Err(error) => {
            println!("Unable to insert note: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => NOTE_ADD_SUCCESS
    };
}

/// Route for updating note
#[put("/", data = "<json_note>")]
fn update(json_note: Json<SavingNote>, token: Token, connection: Connection) -> ApiResponse {
    let note = json_note.to_note(token.sub.clone());
    if !note_exists(&note.account_id, &note.note_id, &*connection) {
        return NOTE_NOT_EXISTS;
    }
    return match note_update_if_exists(note, &*connection) {
        Err(error) => {
            println!("Unable to update note: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => NOTE_UPDATE_SUCCESS
    };
}

#[patch("/<note_id>", data = "<json_note>")]
fn patch(note_id: String, json_note: Json<PatchingNote>, token: Token, connection: Connection) -> ApiResponse {
    let note = json_note.0;
    if note.last_modify_date.is_none() {
        return NOTE_MISSING_LAST_MODIFY;
    }
    if !note_exists(&token.sub, &note_id, &*connection) {
        return NOTE_NOT_EXISTS;
    }
    return match note_patch_if_exists(token.sub, note_id, note, &*connection) {
        Err(error) => {
            println!("Unable to patch note: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => NOTE_PATCH_SUCCESS
    };
}

/// Route for deleting single note identified by id
#[delete("/<note_id>")]
fn delete(note_id: String, token: Token, connection: Connection) -> ApiResponse {
    if !note_exists(&token.sub, &note_id, &*connection) {
        return NOTE_NOT_EXISTS;
    }
    return match note_delete(token.sub, note_id, &*connection) {
        Err(error) => {
            println!("Unable to delete note: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => NOTE_DELETE_SUCCESS
    };
}

/// Route for deleting all notes of an user
#[delete("/all")]
fn delete_all(token: Token, connection: Connection) -> ApiResponse {
    return match note_delete_all(token.sub, &*connection) {
        Err(error) => {
            println!("Unable to delete all notes: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_) => NOTES_DELETE_SUCCESS
    };
}

impl<'r> Responder<'r> for NoteResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(serde_json::to_string(&self).unwrap().respond_to(&req).unwrap())
            .status(Status::Ok)
            .header(ContentType::JSON)
            .ok()
    }
}

/// Route for getting all the notes which are updated after provided timestamp
#[get("/list?<last_updated>")]
fn get_updated(last_updated: i64, token: Token, connection: Connection) -> Result<NoteResponse, ApiResponse> {
    return match notes_get_all(&token.sub, &*connection) {
        Err(error) => {
            println!("Unable to get notes: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(synced_notes) => {
            let last_updated_datetime = Utc.timestamp(last_updated / 1000, 0);
            let mut updated_notes: Vec<Note> = vec![];
            for note in synced_notes {
                if !note.last_modify_date.le(&last_updated_datetime) {
                    updated_notes.push(note);
                }
            }
            Ok(NoteResponse {
                message: NOTE_LIST_SUCCESS.body,
                notes: updated_notes,
            })
        }
    };
}

impl<'r> Responder<'r> for DeletedResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(serde_json::to_string(&self).unwrap().respond_to(&req).unwrap())
            .status(Status::Ok)
            .header(ContentType::JSON)
            .ok()
    }
}

/// Route for getting a list of deleted notes based on a provided list of id's
#[post("/deleted", data = "<id_list>")]
fn get_deleted(id_list: Json<Vec<String>>, token: Token, connection: Connection) -> Result<DeletedResponse, ApiResponse> {
    return match notes_get_existing(&token.sub, id_list.clone(), &*connection) {
        Err(error) => {
            println!("Unable to get deleted id's: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(existing_list) => {
            // Since the repository method can only find what note_id's exist,
            // we need to convert it into a list of non-existing id's
            let deleted_list = id_list.0.into_iter()
                .filter(|id| !existing_list.contains(id)).collect();
            Ok(DeletedResponse {
                deleted: deleted_list
            })
        }
    };
}



pub(crate) const NOTE_NOT_FOUND: ApiResponse = ApiResponse {
    body: "NoteNotFound",
    status: Status::InternalServerError
};
pub(crate) const NOTE_ALREADY_EXISTS: ApiResponse = ApiResponse {
    body: "NoteAlreadyExists",
    status: Status::Conflict
};
pub(crate) const NOTE_NOT_EXISTS: ApiResponse = ApiResponse {
    body: "NoteDoesntExist",
    status: Status::NotFound
};
pub(crate) const NOTE_MISSING_LAST_MODIFY: ApiResponse = ApiResponse {
    body: "NoteMissingLastModifyDate",
    status: Status::BadRequest
};

pub(crate) const NOTE_ADD_SUCCESS: ApiResponse = ApiResponse {
    body: "NoteAddSuccess",
    status: Status::Ok
};
pub(crate) const NOTE_UPDATE_SUCCESS: ApiResponse = ApiResponse {
    body: "NoteUpdateSuccess",
    status: Status::Ok
};
pub(crate) const NOTE_PATCH_SUCCESS: ApiResponse = ApiResponse {
    body: "NotePatchSuccess",
    status: Status::Ok
};
pub(crate) const NOTE_DELETE_SUCCESS: ApiResponse = ApiResponse {
    body: "NoteDeleteSuccess",
    status: Status::Ok
};
pub(crate) const NOTES_DELETE_SUCCESS: ApiResponse = ApiResponse {
    body: "NotesDeleteSuccess",
    status: Status::Ok
};
pub(crate) const NOTE_LIST_SUCCESS: ApiResponse = ApiResponse {
    body: "NoteListSuccess",
    status: Status::Ok
};
pub(crate) const INTERNAL_DB_ERROR: ApiResponse = ApiResponse {
    body: "InternalDatabaseError",
    status: Status::InternalServerError
};