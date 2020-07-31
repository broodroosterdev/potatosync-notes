use std::env;
use rocket::{request, Request};
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket_failure::errors::Status;
use jsonwebtoken::{Validation, decode, DecodingKey};
use serde::Deserialize;
#[cfg(test)]
use mocktopus::macros::*;
/// Claims for access_token
#[derive(Deserialize)]
pub struct Token {
    pub(crate) sub: String,
    pub role: String,
    #[serde(rename = "type")]
    pub(crate) token_type: String,
    pub iat: u64,
    pub exp: u64,
}

/// Get access_token from header and verify it
#[cfg_attr(test, mockable)]
pub fn read_token(token: &str) -> Result<Token, String> {
    let secret = env::var("JWT_SECRET").expect("Could not find JWT_SECRET");
    let validation = Validation::default();
    return match decode::<Token>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation){
        Ok(token_data) => {
            let claims: Token = token_data.claims;
            if !claims.token_type.eq("jwt") {
                return Err("Token is not of type jwt".parse().unwrap())
            }
            Ok(claims)
        },
        Err(error) => Err(error.to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = String;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, "Missing auth token".parse().unwrap()));
        }
        let token = keys[0].replace("Bearer ", "");
        return match read_token(token.as_ref()) {
            Ok(claim) => Outcome::Success(claim),
            Err(err) => {
                println!("{}", err);
                Outcome::Failure((Status::Unauthorized, err))
            },
        };
    }
}