use rocket::http::Status;
use rocket::response::status;
use rocket::Rocket;

/// Used for catching 401 errors e.g. invalid access token
#[catch(401)]
pub fn token_error() -> status::Custom<String> {
    status::Custom(
        Status::Unauthorized,
        String::from("InvalidAuth")
    )
}

//Used for catching 422 errors e.g. missing JSON fields
#[catch(422)]
pub fn unprocessable_entity() -> status::Custom<String> {
    status::Custom(
        Status::UnprocessableEntity,
        String::from("CantProcessEntity")
    )
}

//Used for catching 500 errors e.g. panics
#[catch(500)]
pub fn internal_server_error() -> status::Custom<String> {
    status::Custom(
        Status::InternalServerError,
        String::from("InternalServerError")
    )
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.register(catchers![
        token_error,
        unprocessable_entity,
        internal_server_error
    ])
}