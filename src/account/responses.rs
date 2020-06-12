use crate::error::ApiError;

pub(crate) const INCORRECT_CREDENTIALS: ApiError = ApiError {
    description: "Incorrect login credentials (username/email/password)",
    code: 31,
};
pub(crate) const USER_NOT_VERIFIED: ApiError = ApiError {
    description: "User is not verified",
    code: 32,
};
pub(crate) const INVALID_EMAIL: ApiError = ApiError {
    description: "Invalid email",
    code: 33,
};
pub(crate) const INVALID_USERNAME: ApiError = ApiError {
    description: "Invalid username",
    code: 33,
};
pub(crate) const INVALID_PASSWORD: ApiError = ApiError {
    description: "Invalid password",
    code: 34,
};
pub(crate) const INTERNAL_ERROR: ApiError = ApiError {
    description: "Internal error, ask the administrator to check the logs",
    code: 30
};