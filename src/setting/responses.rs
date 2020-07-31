use rocket::http::Status;
use crate::responders::ApiResponse;

pub(crate) const INVALID_KEY: ApiResponse = ApiResponse {
    body: "InvalidKey",
    status: Status::BadRequest
};
pub(crate) const SETTING_DOESNT_EXIST: ApiResponse = ApiResponse {
    body: "SettingDoesntExist",
    status: Status::NotFound
};

pub(crate) const SETTING_UPDATED: ApiResponse = ApiResponse {
    body: "SettingUpdated",
    status: Status::Ok
};