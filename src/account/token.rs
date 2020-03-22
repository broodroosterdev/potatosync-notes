use std::env;
use std::ops::Add;

use chrono::Duration;
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, select};
use diesel::expression::exists::exists;
use diesel::query_dsl::filter_dsl::FilterDsl;
use jsonwebtokens::{Algorithm, AlgorithmID, Verifier};
use jsonwebtokens::encode;
use jsonwebtokens::error::Error;
use rocket::{request, Request};
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket_failure::errors::Status;
use serde_json::Value;

use crate::db;
use crate::schema::tokens;

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub(crate) sub: String,
    exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TokenJson {
    token: Token
}

pub fn read_token(key: &str) -> Result<Token, String> {
    let access_token_secret = env::var("ACCESS_TOKEN_SECRET").expect("Could not find ACCESS_TOKEN_SECRET in .env");
    let algo = Algorithm::new_hmac(AlgorithmID::HS256, access_token_secret).unwrap();
    let verifier = Verifier::create()
        .leeway(5)    // give this much leeway when validating exp, nbf and iat claims
        .build().unwrap();
    let claims: Result<Value, Error> = verifier.verify(&key, &algo);
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
            return Outcome::Failure((Status::Forbidden, "Missing auth token".parse().unwrap()));
        }
        return match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(claim),
            Err(err) => Outcome::Failure((Status::Forbidden, err))
        };
    }
}

impl Token {
    pub(crate) fn create_access_token(account_id: i32) -> String {
        let access_token_secret = env::var("ACCESS_TOKEN_SECRET").expect("Could not find ACCESS_TOKEN_SECRET in .env");
        let algo = jsonwebtokens::Algorithm::new_hmac(AlgorithmID::HS256, access_token_secret).unwrap();
        let header = json!({
            "alg": algo.name(),
            "typ": "JWT"
        });
        let user = serde_json::to_value(&Token {
            sub: account_id.to_string(),
            exp: chrono::Utc::now().checked_add_signed(Duration::minutes(30)).unwrap().timestamp(),
        }).unwrap();
        let token = encode(&header, &user, &algo).unwrap();
        token
    }
}

#[table_name = "tokens"]
#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone)]
pub struct RefreshTokenDb {
    account_id: i32,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub(crate) sub: String,
}

impl RefreshToken {
    fn create_db(&self, token: String) -> RefreshTokenDb {
        RefreshTokenDb {
            account_id: self.sub.parse().unwrap(),
            token,
        }
    }
    pub(crate) fn create_refresh_token(account_id: i32, connection: &PgConnection) -> String {
        let refresh_token_secret = env::var("REFRESH_TOKEN_SECRET").expect("Could not find REFRESH_TOKEN_SECRET in .env");
        let algo = jsonwebtokens::Algorithm::new_hmac(AlgorithmID::HS256, refresh_token_secret).unwrap();
        let header = json!({
            "alg": algo.name(),
            "typ": "JWT"
        });
        let token = RefreshToken {
            sub: account_id.to_string(),
        };
        let claims = serde_json::to_value(&token).unwrap();

        let jwt = encode(&header, &claims, &algo).unwrap();
        diesel::insert_into(tokens::table).values(token.create_db(jwt.clone())).execute(connection);
        jwt
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenJson {
    pub(crate) token: RefreshToken
}

impl<'a, 'r> FromRequest<'a, 'r> for RefreshToken {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Forbidden, "Missing auth token".parse().unwrap()));
        }
        return match read_refresh_token(keys[0]) {
            Ok(claim) => Outcome::Success(claim),
            Err(err) => Outcome::Failure((Status::Forbidden, err))
        };
    }
}

pub fn read_refresh_token(key: &str) -> Result<RefreshToken, String> {
    let connection = db::connect().get().unwrap();
    let token_exists: Result<bool, diesel::result::Error> = select(exists(tokens::dsl::tokens.filter(tokens::token.eq(key)))).get_result(&connection);
    if !token_exists.ok().unwrap() {
        return Err("Token is expired".parse().unwrap());
    }
    let refresh_token_secret = env::var("REFRESH_TOKEN_SECRET").expect("Could not find REFRESH_TOKEN_SECRET in .env");
    let algo = Algorithm::new_hmac(AlgorithmID::HS256, refresh_token_secret).unwrap();
    let verifier = Verifier::create()
        .leeway(5)    // give this much leeway when validating exp, nbf and iat claims
        .build().unwrap();
    let claims: Result<Value, Error> = verifier.verify(&key, &algo);
    return if claims.is_ok() {
        let refresh_token: RefreshToken = serde_json::from_value(claims.unwrap()).unwrap();
        Ok(refresh_token)
    } else {
        println!("Token Error");
        Err(claims.err().unwrap().to_string())
    };
}