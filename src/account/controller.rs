use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Local, SecondsFormat};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, select};
use diesel::expression::exists::exists;
use diesel::query_dsl::filter_dsl::FilterDsl;
use jsonwebtokens::AlgorithmID;

use crate::account::model::{Account, LoginCredentials, NewAccount, NewDBAccount, TokenAccount, TokenResponse, Password, InfoResponse, Username, validate_password, validate_username};
use crate::account::token::{RefreshToken, Token};
use crate::account::token;
use crate::status_response::{StatusResponse, ApiResponse};
use crate::schema::accounts;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rocket_failure::errors::Status;
use diesel::result::Error;

pub(crate) fn login(credentials: LoginCredentials, connection: &PgConnection) -> Result<TokenResponse, StatusResponse> {
    let account: Account;
    if credentials.email.is_some() {
        let email = credentials.email.clone().unwrap().clone();
        let email_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::email.eq(email.clone())))).get_result(connection);
        if !email_exists.ok().unwrap() {
            return Err(StatusResponse::new("Email address not found".parse().unwrap(), false));
        }
        account = accounts::dsl::accounts.filter(accounts::email.eq(email.clone())).first::<Account>(connection).unwrap();
    } else if credentials.username.is_some() {
        let username = credentials.username.clone().unwrap().clone();
        let username_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::username.eq(username.clone())))).get_result(connection);
        if !username_exists.ok().unwrap() {
            return Err(StatusResponse::new("UsernameNotFoundError".parse().unwrap(), false));
        }
        account = accounts::dsl::accounts.filter(accounts::username.eq(username.clone())).first::<Account>(connection).unwrap();
    } else {
        return Err(StatusResponse::new("Both username and password missing".parse().unwrap(), false));
    }
    let access_token = Token::create_access_token(account.id, account.password_identifier.clone());
    let refresh_token = RefreshToken::create_refresh_token(account.id, connection);
    Ok(TokenResponse::new(account, access_token, refresh_token))
}


pub(crate) fn create(account: NewAccount, connection: &PgConnection) -> ApiResponse {
    let is_valid: StatusResponse = account.is_valid(connection);
    if !is_valid.status {
        return ApiResponse{
            json:  is_valid.to_string(),
            status: Status::BadRequest
        };
    }
    let hashed_password = hash(account.password.clone(), 10).expect("Error hashing password: ");
    let password_identifier: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    let account = NewDBAccount {
        created_at: Local::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        updated_at: None,
        deleted_at: None,
        email: account.email.clone(),
        username: account.username.clone(),
        password: hashed_password,
        image_url: "".to_string(),
        password_identifier: password_identifier.clone()
    };
    let insert_result: Result<Account, Error> = diesel::insert_into(accounts::table)
        .values(&account)
        .returning(accounts::all_columns)
        .get_result(connection);
    if insert_result.is_err() {
        return ApiResponse{
            json: StatusResponse::new("Could not insert account into DB".parse().unwrap(), false).to_string(),
            status: Status::BadRequest
        };
    }
    let account = insert_result.unwrap();
    let access_token = Token::create_access_token(account.id, password_identifier.clone());
    let refresh_token = RefreshToken::create_refresh_token(account.id, connection);
    let json = TokenResponse {
        message: "AccCreationSuccess".parse().unwrap(),
        status: true,
        account: TokenAccount::from_account(account, access_token, refresh_token),
    }.to_string();
    return ApiResponse{
        json,
        status: Status::Ok
    }
}

pub(crate) fn change_password(account_id: i32, password: Password, connection: &PgConnection) -> String {
    let new_password = password.password;
    let password_valid = validate_password(new_password.clone());
    if password_valid.is_err() {
        return password_valid.err().unwrap().to_string();
    }
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string();
    }
    let hashed_password = hash(new_password.clone(), 10).unwrap();
    let update_result = diesel::update(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))
        .set(accounts::password.eq(hashed_password))
        .execute(connection);
    return if update_result.is_err() {
        StatusResponse::new(update_result.err().unwrap().to_string(), false).to_string()
    } else {
        StatusResponse::new("UpdateSuccess".parse().unwrap(), true).to_string()
    };
}

pub(crate) fn change_username(account_id: i32, username: Username, connection: &PgConnection) -> String {
    let new_username = username.username;
    let username_valid = validate_username(new_username.clone(), connection);
    if username_valid.is_err() {
        return username_valid.err().unwrap().to_string();
    }
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string();
    }
    let update_result = diesel::update(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))
        .set(accounts::username.eq(new_username))
        .execute(connection);
    return if update_result.is_err() {
        StatusResponse::new(update_result.err().unwrap().to_string(), false).to_string()
    } else {
        StatusResponse::new("UpdateSuccess".parse().unwrap(), true).to_string()
    };
}

pub(crate) fn get_info(account_id: i32, connection: &PgConnection) -> String {
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string();
    }
    let account = accounts::dsl::accounts.filter(accounts::id.eq(account_id)).first::<Account>(connection).unwrap();
    return InfoResponse::new(account).to_string();
}
/*
pub(crate) fn save_image(account_id: i32, image_url: String, connection: &PgConnection) -> String {

    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string();
    }
}*/