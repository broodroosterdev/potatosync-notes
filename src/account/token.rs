use std::env;

use chrono::Duration;
use diesel::PgConnection;
use rocket::{request, Request};
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket_failure::errors::Status;
use tokkit::client::TokenInfoServiceClientBuilder;

use crate::account::repository::get_account_by_id;
use crate::db;
use crate::status_response::ApiResponse;
use crate::status_response::StatusResponse;

/// Claims for access_token
#[derive(Serialize, Deserialize)]
pub struct Token {
    pub(crate) sub: String,
    exp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct IntrospectInfo {
    token: &'static str,
    client_secret: &'static str,
    client_id: &'static str,
}

/// Get access_token from header and verify it
pub fn read_token(key: &str) -> Result<Token, String> {
    let params = [
        ("client_secret", env::var("CLIENT_SECRET").expect("Could not find CLIENT_SECRET in .env")),
        ("client_id", env::var("CLIENT_ID").expect("Could not find CLIENT_ID in .env")),
        ("token", key.to_string())
    ];
    let client = reqwest::blocking::Client::new();
    let res: String = client.post(&env::var("INTROSPECT_URL").expect("Could not find INTROSPECT_URL in .env")).form(&params).send().unwrap().text().unwrap();
    println!("{}", res);
    let info = tokkit::parsers::parse(res.as_bytes(), Some("active"), None, None, None).ok().unwrap();
    println!("{:?}", info);
    return if info.active {
        let info = tokkit::parsers::parse(res.as_bytes(), Some("active"), Some("sub"), Some("scope"), Some("exp")).ok().unwrap();
        println!("{:?}", info);
        Ok(Token {
            sub: info.user_id.unwrap().0,
            exp: info.expires_in_seconds.unwrap()
        })
    } else {
        println!("TOKEN IS NOT ACTIVE AAAH");
        Err("Token is not active".parse().unwrap())
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

/*
/// Claims of the refresh token
#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub(crate) sub: String,
    pub(crate) password_id: String
}

impl RefreshToken {
    /// Creates new refresh token with account_id and password identifier
    pub(crate) fn create_refresh_token(account_id: String, password_identifier: String) -> String {
        let refresh_token_secret = env::var("REFRESH_TOKEN_SECRET").expect("Could not find REFRESH_TOKEN_SECRET in .env");
        let token = RefreshToken {
            sub: account_id.clone(),
            password_id: password_identifier,
        };
        let algo = Algorithm::new_hmac(AlgorithmID::HS256, refresh_token_secret).unwrap();
        let header = json!({"alg": algo.name()});
        let token = encode(&header, &token, &algo).unwrap();
        token
    }
}

/// Struct used for getting refresh_token from client when refreshing
#[derive(Serialize, Deserialize)]
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
        let account = get_account_by_id(refresh_token.sub.clone(), &connection);
        if account.password_identifier.eq(&refresh_token.password_id) {
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
#[derive(Serialize, Deserialize)]
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
    let account = get_account_by_id(refresh_token.sub.clone(), connection);
    return if account.password_identifier.eq(&refresh_token.password_id) {
        let refresh_response = RefreshResponse {
            message: "RefreshSuccess".parse().unwrap(),
            status: true,
            token: "".to_string(),
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

#[cfg(test)]
mod tests{
    use crate::account::token::read_token;

    #[test]
    fn can_introspect_token(){
        let token = read_token("eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICIweTUyYzVxamh1bmtPUzFEZ3o1OF9UR1JPNWpqNUplVTdQdEVueVhubks4In0.eyJleHAiOjE1OTIzMzYzMDIsImlhdCI6MTU5MjMzMjcwMiwiYXV0aF90aW1lIjoxNTkyMzMxNzMxLCJqdGkiOiIyMDAxOTk0Yi02ZDhlLTQ3ZDUtOTFlOS1iNDI4MDNlMDA0ZmYiLCJpc3MiOiJodHRwOi8vbG9jYWxob3N0OjgwODAvYXV0aC9yZWFsbXMvcG90YXRvc3luYyIsImF1ZCI6ImFjY291bnQiLCJzdWIiOiI5ZTM3ZDc1Mi1mMDdkLTQ1ODEtODQwNS05MTY4Y2ZhOGIwZDciLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiJub3RlcyIsInNlc3Npb25fc3RhdGUiOiJlNGRlNTA1ZS1iMzhlLTQ2NjUtYjVmYS1mMDA3NmU4OGE2YzgiLCJhY3IiOiIwIiwicmVhbG1fYWNjZXNzIjp7InJvbGVzIjpbIm9mZmxpbmVfYWNjZXNzIiwidW1hX2F1dGhvcml6YXRpb24iXX0sInJlc291cmNlX2FjY2VzcyI6eyJub3RlcyI6eyJyb2xlcyI6WyJub3RlcyJdfSwiYWNjb3VudCI6eyJyb2xlcyI6WyJtYW5hZ2UtYWNjb3VudCIsIm1hbmFnZS1hY2NvdW50LWxpbmtzIiwidmlldy1wcm9maWxlIl19fSwic2NvcGUiOiJvcGVuaWQgcHJvZmlsZSBlbWFpbCIsImVtYWlsX3ZlcmlmaWVkIjpmYWxzZSwicHJlZmVycmVkX3VzZXJuYW1lIjoiYmFzIn0.bejCtimPVmIje6QL63uXwwFBhJaOj0dwrazBoCmQAfi_bN9PuLMN46m-x50RuSRFpxBpZGcblwHm7jca1OxJnAnU2pPjfiaHRno17qgfYdMW-ZVe_LlAwSaEle3H4ini1DSBxYVP15CJ6Lo2MhVJk8b-MkQqIEoIAYs4_WEpAykIHHbs_O9sBV-5Mhnwo8KuzgDq_vj3nXANCJIDNwFZZGPCA_aIT7goyN4ZRWWv35HjQqhtn8WVc2qVvxJHg4xt_33ZsYP9NMlwLBUgd2p3zCMrwDqS8tD7fdvanFUcY5ZPU77Y9CJoISoc7g3diJWiHf2sC1vdfVrlX7kR2mlkvw");
        assert!(token.is_ok());
        println!("{}", token.ok().unwrap().sub);
    }
}

*/