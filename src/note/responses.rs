use crate::responders::ApiResponse;
use rocket::http::Status;

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