use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, SecondsFormat, Utc};
use diesel;
use diesel::expression::exists::{exists, Exists};
use diesel::prelude::*;
use diesel::select;
use jsonwebtokens::*;
use jsonwebtokens::AlgorithmID;
use serde_derive::*;

use crate::account::controller::*;
use crate::account::token::Token;
use crate::response::StatusResponse;
use crate::schema::accounts;

#[table_name = "accounts"]
#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone)]
pub struct Account {
    pub(crate) id: i32,
    created_at: String,
    updated_at: Option<String>,
    deleted_at: Option<String>,
    email: String,
    username: String,
    pub(crate) password: String,
    image_url: String,
}

#[table_name = "accounts"]
#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct NewDBAccount {
    pub(crate) created_at: String,
    pub(crate) updated_at: Option<String>,
    pub(crate) deleted_at: Option<String>,
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) image_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginCredentials {
    pub(crate) email: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewAccount {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl NewAccount {
    pub(crate) fn is_valid(&self, connection: &PgConnection) -> StatusResponse {
        if !self.email.contains("@") {
            return StatusResponse::new("MalformedEmailError".parse().unwrap(), false);
        }

        if self.username.chars().count() <= 4 || self.username.chars().count() > 60 {
            return StatusResponse::new("UsernameOutOfBoundsError".parse().unwrap(), false);
        }

        if self.password.chars().count() < 8 || self.password.chars().count() > 60 {
            return StatusResponse::new("PassOutOfBoundsError".parse().unwrap(), false);
        }
        let email_exists = select(exists(accounts::dsl::accounts.filter(accounts::email.eq(self.email.clone())))).get_result(connection);
        if email_exists.ok().unwrap() {
            return StatusResponse::new("EmailAlreadyExistsError".parse().unwrap(), false);
        }

        let username_exists = select(exists(accounts::dsl::accounts.filter(accounts::username.eq(self.username.clone())))).get_result(connection);
        if username_exists.ok().unwrap() {
            return StatusResponse::new("UsernameAlreadyExistsError".parse().unwrap(), false);
        }

        return StatusResponse::new("ValidationSuccess".parse().unwrap(), true);
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub(crate) message: String,
    pub(crate) status: bool,
    pub(crate) account: TokenAccount,
}

impl TokenResponse {
    pub(crate) fn new(account: Account, access_token: String, refresh_token: String) -> Self {
        TokenResponse {
            message: "LoginSuccess".parse().unwrap(),
            status: true,
            account: TokenAccount::from_account(account, access_token, refresh_token),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct TokenAccount {
    created_at: String,
    updated_at: Option<String>,
    deleted_at: Option<String>,
    email: String,
    username: String,
    password: String,
    image_url: String,
    refresh_token: String,
    access_token: String,
}

impl TokenAccount {
    pub(crate) fn from_account(account: Account, access_token: String, refresh_token: String) -> TokenAccount {
        TokenAccount {
            created_at: account.created_at.clone(),
            updated_at: account.updated_at,
            deleted_at: account.deleted_at,
            email: account.email,
            username: account.username,
            password: "".parse().unwrap(),//Keep the password empty to avoid accidentally returning it to the client
            image_url: account.image_url,
            refresh_token,
            access_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use crate::account::token::{read_refresh_token, read_token};
    use crate::db;

    use super::*;

    #[test]
    fn check_validation() {
        let connection = db::connect().get().unwrap();
        let new_account = NewAccount {
            email: "test3@test.com".to_string(),
            username: "broodrooster3".to_string(),
            password: "broodrooster".to_string(),
        };
        let valid = new_account.is_valid(&connection);
        if valid.status != true {
            println!("{}", valid.message);
        }
    }

    #[test]
    fn check_creation() {
        let connection = db::connect().get().unwrap();
        check_validation();
        let new_account = NewAccount {
            email: "test3@test.com".to_string(),
            username: "broodrooster3".to_string(),
            password: "broodrooster".to_string(),
        };
        let create_result = create(new_account, &connection);
        println!("{}", create_result);
    }

    #[test]
    fn check_login() {
        let connection = db::connect().get().unwrap();
        let credentials = LoginCredentials {
            email: Some("test3@test.com".to_string()),
            username: None,
            password: "broodrooster".to_string(),
        };
        let login_result = login(credentials, &connection);
        println!("{}", login_result);
    }

    #[test]
    fn check_expire() {
        let connection = db::connect().get().unwrap();
        let credentials = LoginCredentials {
            email: Some("test3@test.com".to_string()),
            username: None,
            password: "broodrooster".to_string(),
        };
        let login_result = login(credentials, &connection);
        let token_response: TokenResponse = serde_json::from_str(login_result.as_str()).unwrap();
        println!("{}", login_result);
        println!("Waiting for 20 seconds...");
        thread::sleep(Duration::from_secs(20));
        println!("{}", read_token(token_response.account.access_token.as_ref()).err().unwrap());
    }
}

