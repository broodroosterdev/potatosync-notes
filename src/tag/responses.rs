use crate::responders::ApiResponse;
use rocket::http::Status;

pub(crate) const TAG_ALREADY_EXISTS: ApiResponse = ApiResponse {
    body: "TagAlreadyExists",
    status: Status::BadRequest
};

pub(crate) const TAG_MISSING_LAST_MODIFY: ApiResponse = ApiResponse {
    body: "TagMissingLastModifyDate",
    status: Status::BadRequest
};

pub(crate) const TAG_DOESNT_EXIST: ApiResponse = ApiResponse {
    body: "TagDoesntExist",
    status: Status::BadRequest
};

pub(crate) const TAG_ADD_SUCCESS: ApiResponse = ApiResponse {
    body: "TagAddSuccess",
    status: Status::Ok
};

pub(crate) const TAG_PATCH_SUCCESS: ApiResponse = ApiResponse {
    body: "TagPatchSuccess",
    status: Status::Ok
};

pub(crate) const TAG_DELETE_SUCCESS: ApiResponse = ApiResponse {
    body: "TagDeleteSuccess",
    status: Status::Ok
};