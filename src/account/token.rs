use std::env;
use std::ops::Add;

use chrono::Duration;
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, select};
use diesel::expression::exists::exists;
use diesel::query_dsl::filter_dsl::FilterDsl;
use jsonwebtokens::{Algorithm, AlgorithmID, encode, Verifier};
use jsonwebtokens::error::Error;
use rocket::{request, Request};
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket_failure::errors::Status;
use serde_json::Value;

use crate::account::controller::get_account_by_id;
use crate::account::model::Account;
use crate::db;
use crate::schema::tokens;
use crate::status_response::ApiResponse;
use crate::status_response::StatusResponse;

/// Claims for access_token
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Token {
    pub(crate) sub: String,
    exp: i64,
}

/// Get access_token from header and verify it
pub fn read_token(key: &str) -> Result<Token, String> {
    let access_token_secret = env::var("ACCESS_TOKEN_SECRET").expect("Could not find ACCESS_TOKEN_SECRET in .env");
    let algo = Algorithm::new_hmac(AlgorithmID::HS256, access_token_secret).unwrap();
    let verifier = Verifier::create()
        .leeway(5)
        .build().unwrap();
    let claims = verifier.verify(&key, &algo);
    return if claims.is_ok() {
        let user: Token = serde_json::from_value(claims.unwrap()).unwrap();
        Ok(user)
    } else {
        println!("Token Error");
        let error = claims.err().unwrap().to_string();
        println!("{}", error);
        Err(error)
    };
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, "Missing auth token".parse().unwrap()));
        }
        return match read_token(keys[0].replace("Bearer ", "").as_ref()) {
            Ok(claim) => Outcome::Success(claim),
            Err(err) => Outcome::Failure((Status::Unauthorized, err))
        };
    }
}

impl Token {
    /// Create new access_token for certain account_id
    pub(crate) fn create_access_token(account_id: i32) -> String {
        let access_token_secret = env::var("ACCESS_TOKEN_SECRET").expect("Could not find ACCESS_TOKEN_SECRET in .env");
        let user = serde_json::to_value(&Token {
            sub: account_id.to_string(),
            exp: chrono::Utc::now().checked_add_signed(Duration::minutes(15)).unwrap().timestamp(),
        }).unwrap();
        let algo = Algorithm::new_hmac(AlgorithmID::HS256, access_token_secret).unwrap();
        let header = json!({"alg": algo.name()});
        let token = encode(&header, &user, &algo).unwrap();
        token
    }
}

/// Used for storing token in DB
#[table_name = "tokens"]
#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RefreshTokenDb {
    account_id: i32,
    token: String,
}

/// Claims of the refresh token
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RefreshToken {
    pub(crate) sub: String,
    pub(crate) pwId: String
}

impl RefreshToken {
    /// Turns RefreshToken in RefreshTokenDb to store it in db
    fn create_db(&self, token: String) -> RefreshTokenDb {
        RefreshTokenDb {
            account_id: self.sub.parse().unwrap(),
            token,
        }
    }
    /// Creates new refresh token with account_id and password identifier
    pub(crate) fn create_refresh_token(account_id: i32, password_identifier: String, connection: &PgConnection) -> String {
        let refresh_token_secret = env::var("REFRESH_TOKEN_SECRET").expect("Could not find REFRESH_TOKEN_SECRET in .env");
        let token = RefreshToken {
            sub: account_id.to_string(),
            pwId: password_identifier,
        };
        let algo = Algorithm::new_hmac(AlgorithmID::HS256, refresh_token_secret).unwrap();
        let header = json!({"alg": algo.name()});
        let token = encode(&header, &token, &algo).unwrap();
        token
    }
}

/// Struct used for getting refresh_token from client when refreshing
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RefreshTokenJson {
    pub(crate) token: String
}

/// Verify refresh token
pub fn read_refresh_token(key: &str) -> Result<RefreshToken, String> {
    let refresh_token_secret = env::var("REFRESH_TOKEN_SECRET").expect("Could not find REFRESH_TOKEN_SECRET in .env");
    let algo = Algorithm::new_hmac(AlgorithmID::HS256, refresh_token_secret).unwrap();
    let verifier = Verifier::create()
        .leeway(5)
        .build().unwrap();
    let claims = verifier.verify(&key, &algo);
    return if claims.is_ok() {
        let refresh_token: RefreshToken = serde_json::from_value(claims.unwrap()).unwrap();
        let connection = db::connect().get().unwrap();
        let account = get_account_by_id(refresh_token.sub.parse().unwrap(), &connection);
        if account.password_identifier.eq(&refresh_token.pwId) {
            Ok(refresh_token)
        } else {
            Err("Password identifier does not match".parse().unwrap())
        }
    } else {
        println!("Token Error");
        Err(claims.err().unwrap().to_string())
    };
}

/// Struct used for sending back a new access_token with status of the request
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RefreshResponse {
    message: String,
    status: bool,
    token: String,
}

impl ToString for RefreshResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// Refreshes the access token.
/// Also checks if the password identifier matches with the one in the DB
pub(crate) fn refresh_token(refresh_token: RefreshToken, connection: &PgConnection) -> ApiResponse {
    let account = get_account_by_id(refresh_token.sub.parse().unwrap(), connection);
    return if account.password_identifier.eq(&refresh_token.pwId) {
        let refresh_response = RefreshResponse {
            message: "RefreshSuccess".parse().unwrap(),
            status: true,
            token: Token::create_access_token(refresh_token.sub.parse().unwrap()),
        };
        ApiResponse {
            json: refresh_response.to_string(),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            json: StatusResponse::new("Password identifier does not match".parse().unwrap(), false).to_string(),
            status: Status::BadRequest,
        }
    }
}