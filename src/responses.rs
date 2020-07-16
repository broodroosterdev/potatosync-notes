use crate::responders::ApiError;

pub(crate) const JSON_INVALID: ApiError = ApiError {
    description: "JsonInvalid",
    code: 2
};