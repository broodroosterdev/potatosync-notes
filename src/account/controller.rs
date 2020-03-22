use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Local, SecondsFormat};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, select};
use diesel::expression::exists::exists;
use diesel::query_dsl::filter_dsl::FilterDsl;
use jsonwebtokens::AlgorithmID;

use crate::account::model::{Account, LoginCredentials, NewAccount, NewDBAccount, TokenAccount, TokenResponse};
use crate::account::token::{RefreshToken, Token};
use crate::account::token;
use crate::response::StatusResponse;
use crate::schema::accounts;

pub(crate) fn login(credentials: LoginCredentials, connection: &PgConnection) -> String {
    let account: Account;
    if credentials.email.is_some() {
        let email = credentials.email.clone().unwrap().clone();
        let email_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::email.eq(email.clone())))).get_result(connection);
        if !email_exists.ok().unwrap() {
            return StatusResponse::new("Email address not found".parse().unwrap(), false).to_string();
        }
        account = accounts::dsl::accounts.filter(accounts::email.eq(email.clone())).first::<Account>(connection).unwrap();
    } else if credentials.username.is_some() {
        let username = credentials.username.clone().unwrap().clone();
        let username_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::username.eq(username.clone())))).get_result(connection);
        if !username_exists.ok().unwrap() {
            return StatusResponse::new("UsernameNotFoundError".parse().unwrap(), false).to_string();
        }
        account = accounts::dsl::accounts.filter(accounts::username.eq(username.clone())).first::<Account>(connection).unwrap();
    } else {
        return serde_json::to_string(&StatusResponse::new("Both username and password missing".parse().unwrap(), false)).unwrap();
    }
    let access_token = Token::create_access_token(account.id);
    let refresh_token = RefreshToken::create_refresh_token(account.id, connection);
    serde_json::to_string(&TokenResponse::new(account, access_token, refresh_token)).unwrap()
}


pub(crate) fn create(account: NewAccount, connection: &PgConnection) -> String {
    println!("Checking validity");
    let is_valid: StatusResponse = account.is_valid(connection);
    if !is_valid.status {
        return serde_json::to_string(&is_valid).unwrap();
    }
    println!("Valid!");
    println!("Now hashing password");
    let hashed_password = hash(account.password.clone(), 10).unwrap();
    println!("Hashing completed!");
    println!("Now creating db struct");
    let account = NewDBAccount {
        created_at: Local::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        updated_at: None,
        deleted_at: None,
        email: account.email.clone(),
        username: account.username.clone(),
        password: hashed_password,
        image_url: "".to_string(),
    };
    println!("Inserting into postgres");
    let account: Account = diesel::insert_into(accounts::table)
        .values(&account)
        .returning(accounts::all_columns)
        .get_result(connection).unwrap();
    println!("Creating access token");
    let access_token = Token::create_access_token(account.id);
    println!("Creating refresh token");
    let refresh_token = RefreshToken::create_refresh_token(account.id, connection);
    println!("Creating response");
    serde_json::to_string(&TokenResponse {
        message: "AccCreationSuccess".parse().unwrap(),
        status: true,
        account: TokenAccount::from_account(account, access_token, refresh_token),
    }).unwrap()
}

