use crate::responders::{ApiError, ApiSuccess};

pub(crate) const NOTE_NOT_FOUND: ApiError = ApiError {
    description: "NoteNotFound",
    code: 21
};
pub(crate) const NOTE_EXISTS: ApiError = ApiError {
    description: "NoteAlreadyExists",
    code: 22
};
pub(crate) const NOTE_NOT_EXISTS: ApiError = ApiError {
    description: "NoteDoesntExist",
    code: 23
};
pub(crate) const NOTE_MISSING_LAST_MODIFY: ApiError = ApiError {
    description: "NoteMissingLastModifyDate",
    code: 24
};
pub(crate) const INTERNAL_DB_ERROR: ApiError = ApiError {
    description: "InternalDatabaseError",
    code: 25
};

pub(crate) const NOTE_ADD_SUCCESS: ApiSuccess = ApiSuccess {
    description: "NoteAddSuccess",
    code: 11
};
pub(crate) const NOTE_UPDATE_SUCCESS: ApiSuccess = ApiSuccess {
    description: "NoteUpdateSuccess",
    code: 12
};
pub(crate) const NOTE_PATCH_SUCCESS: ApiSuccess = ApiSuccess {
    description: "NotePatchSuccess",
    code: 13
};
pub(crate) const NOTE_DELETE_SUCCESS: ApiSuccess = ApiSuccess {
    description: "NoteDeleteSuccess",
    code: 14
};
pub(crate) const NOTES_DELETE_SUCCESS: ApiSuccess = ApiSuccess {
    description: "NotesDeleteSuccess",
    code: 15
};
pub(crate) const NOTE_LIST_SUCCESS: ApiSuccess = ApiSuccess {
    description: "NoteListSuccess",
    code: 16
};