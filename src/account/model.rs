use chrono::{DateTime, TimeZone, Utc};
use chrono::serde::ts_milliseconds::*;
use diesel;
use diesel::expression::exists::exists;
use diesel::prelude::*;
use diesel::select;
#[cfg(test)]
use mocktopus::macros::*;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serializer};
use serde_derive::*;

use crate::account::repository::{email_exists, username_exists};
use crate::account::responses::{INVALID_EMAIL, INVALID_PASSWORD, INVALID_USERNAME};
use crate::responses::ApiError;
use crate::schema::accounts;
use crate::schema::reset_tokens;
use crate::schema::verification_tokens;
use crate::status_response::StatusResponse;

/// Special function to serialize Option<DateTime<Utc>>
pub fn serialize_option<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    return if dt.is_some() {
        serializer.serialize_i64(dt.unwrap().timestamp())
    } else {
        serializer.serialize_none()
    }
}

/// Special function to deserialize Option<DateTime<Utc>>
pub fn deserialize_option<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where D: Deserializer<'de>
{
    let dt: Option<i64> = Option::deserialize(d)?;
    if let Some(dt) = dt {
        return Ok(Some(
            Utc.timestamp_opt(dt / 1000,
                              ((dt % 1000) * 1_000_000) as u32).unwrap()
        ));
    }
    Ok(None)
}

/// General Account struct used for retrieving from DB and updating
#[table_name = "accounts"]
#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone)]
pub struct Account {
    pub(crate) id: String,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub(crate) created_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_option")]
    #[serde(serialize_with = "serialize_option")]
    pub(crate) updated_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_option")]
    #[serde(serialize_with = "serialize_option")]
    pub(crate) deleted_at: Option<DateTime<Utc>>,
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) image_url: String,
    pub(crate) password_identifier: String,
    pub(crate) verified: bool,
    pub(crate) shared_prefs: String,
}

#[cfg(test)]
impl Account {
    pub(crate) fn mock_empty() -> Account{
        return Account{
            id: "".to_string(),
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            image_url: "".to_string(),
            password_identifier: "".to_string(),
            verified: false,
            shared_prefs: "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PatchingAccount {
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) image_url: Option<String>,
    pub(crate) shared_prefs: Option<String>,
}

/*
/// Struct used for adding new accounts to the DB. Note the missing id field since the database will provide it for us
#[table_name = "accounts"]
#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct NewDBAccount {
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub(crate) created_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_option")]
    #[serde(serialize_with = "serialize_option")]
    pub(crate) updated_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_option")]
    #[serde(serialize_with = "serialize_option")]
    pub(crate) deleted_at: Option<DateTime<Utc>>,
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) image_url: String,
    pub(crate) password_identifier: String,
    pub(crate) verified: bool,
    pub(crate) shared_prefs: String,
}*/

/// Used for getting the credentials from the client
#[derive(Serialize, Deserialize, Clone)]
pub struct LoginCredentials {
    pub(crate) email: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) password: String,
}

/// Used when changing password
#[derive(Serialize, Deserialize, Clone)]
pub struct Password {
    pub(crate) password: String
}

/// Used when changing username
#[derive(Serialize, Deserialize, Clone)]
pub struct Username {
    pub(crate) username: String
}

/// Used when changing Profile picture
#[derive(Serialize, Deserialize, Clone)]
pub struct Image {
    pub(crate) image: String
}

/// Used for verifying if the entered info when registering is valid
#[derive(Serialize, Deserialize)]
pub struct NewAccount {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl NewAccount {
    /// Checks if account is valid and can be created
    pub(crate) fn is_valid(&self, connection: &PgConnection) -> Result<(), ApiError> {
        let email_valid = email_is_valid(&self.email);
        if !email_valid {
            return Err(INVALID_EMAIL);
        }
        let username_valid = username_is_valid(&self.username);
        if !username_valid {
            return Err(INVALID_USERNAME);
        }
        let password_valid = password_is_valid(&self.password);
        if !password_valid {
            return Err(INVALID_PASSWORD);
        }
        return Ok(());
    }
}

/// Checks if email is valid
#[cfg_attr(test, mockable)]
pub fn email_is_valid(email: &String) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    return email_regex.is_match(email.clone().as_str());
}


/// Checks if username is valid
#[cfg_attr(test, mockable)]
pub fn username_is_valid(username: &String) -> bool {
    return username.chars().count() >= 4 && username.chars().count() < 60;
}

/// Checks if password is valid
#[cfg_attr(test, mockable)]
pub fn password_is_valid(password: &String) -> bool {
    return password.chars().count() > 8 && password.chars().count() < 64;
}

/// Struct used for sending back account info when you have logged in
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
            account: TokenAccount::from_account(account, Some(access_token), Some(refresh_token)),
        }
    }
}

impl ToString for TokenResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// Struct used for returning account info in TokenResponse
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAccount {
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    created_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_option")]
    #[serde(serialize_with = "serialize_option")]
    updated_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_option")]
    #[serde(serialize_with = "serialize_option")]
    deleted_at: Option<DateTime<Utc>>,
    email: String,
    username: String,
    image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<String>,
}

impl TokenAccount {
    pub(crate) fn from_account(account: Account, access_token: Option<String>, refresh_token: Option<String>) -> TokenAccount {
        TokenAccount {
            created_at: account.created_at.clone(),
            updated_at: account.updated_at,
            deleted_at: account.deleted_at,
            email: account.email,
            username: account.username,
            image_url: account.image_url,
            refresh_token,
            access_token,
        }
    }
}

/// Struct used when returning account info to client
#[derive(Serialize, Deserialize)]
pub struct InfoResponse {
    pub(crate) message: String,
    pub(crate) status: bool,
    pub(crate) account: Account,
}

impl InfoResponse {
    pub(crate) fn new(mut account: Account) -> Self {
        account.password = "".parse().unwrap();
        InfoResponse{
            message: "UserFound".parse().unwrap(),
            status: true,
            account
        }
    }
}

impl ToString for InfoResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// Struct used for storing the password reset tokens
#[table_name = "reset_tokens"]
#[derive(Insertable, Queryable, Serialize, Clone)]
pub struct ResetToken {
    pub(crate) account_id: String,
    pub(crate) reset_token: String,
    pub(crate) expires_at: DateTime<Utc>
}

/// Used for storing verification token in DB
#[table_name = "verification_tokens"]
#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
pub struct VerificationToken {
    pub(crate) account_id: String,
    pub(crate) verification_token: String,
    pub(crate) expires_at: DateTime<Utc>,
}

/// Used for storing active session tokens
/*#[table_name = "session_tokens"]
#[derive(Insertabl, Queryable, Serialize, Clone)]
pub struct SessionToken {
    pub(crate) account_id: String,
    pub(crate) session_token: String,
    pub(crate) expires_at: DateTime<Utc>
}*/

#[cfg(test)]
mod tests {
    use mocktopus::mocking::{Mockable, MockResult};

    use crate::account::model::{email_is_valid, password_is_valid, username_is_valid};
    use crate::account::repository::email_exists;
    use crate::db;

    #[test]
    fn give_error_when_email_malformed(){
        dotenv::dotenv().ok();
        let email = "testexample.com".to_string();
        let is_valid = email_is_valid(&email);
        assert_eq!(is_valid, false);
        let email = "test@example.".to_string();
        let is_valid = email_is_valid(&email);
        assert_eq!(is_valid, false);
    }

    #[test]
    fn give_ok_when_email_correct(){
        dotenv::dotenv().ok();
        let email = "test@example.com".to_string();
        let is_valid = email_is_valid(&email);
        assert!(is_valid);
    }

    #[test]
    fn give_error_when_username_out_of_bounds(){
        let username = "test".repeat(20);
        let is_valid = username_is_valid(&username);
        assert_eq!(is_valid, false)
    }

    #[test]
    fn give_ok_when_username_correct(){
        let username = "test".to_string();
        let is_valid = username_is_valid(&username);
        assert!(is_valid);
    }

    #[test]
    fn give_error_when_password_out_of_bounds(){
        let password = "test".repeat(20);
        let is_valid = password_is_valid(&password);
        assert_eq!(is_valid, false);
    }

    #[test]
    fn give_ok_when_password_correct(){
        let password = "testingpassword".to_string();
        let is_valid = password_is_valid(&password);
        assert!(is_valid);
    }
}
