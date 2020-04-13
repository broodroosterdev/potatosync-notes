use std::env;

use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Local, SecondsFormat, Utc};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, select};
use diesel::expression::exists::exists;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::result::Error;
use openssl::sha::sha1;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::response::status::BadRequest;
use rocket_failure::errors::Status;

use crate::account::email::{create_token_email, send_email, VerificationToken};
use crate::account::model::{Account, InfoResponse, LoginCredentials, NewAccount, NewDBAccount, Password, PatchingAccount, TokenAccount, TokenResponse, Username, validate_password, validate_username};
use crate::account::token::{RefreshToken, Token};
use crate::account::token;
use crate::schema::accounts;
use crate::schema::tokens;
use crate::status_response::{ApiResponse, StatusResponse};

/// Used to login user using DB and returns Error if credentials are incorrect
pub(crate) fn login(credentials: LoginCredentials, connection: &PgConnection) -> Result<TokenResponse, StatusResponse> {
    let get_account_result: Result<Account, diesel::result::Error>;
    if credentials.email.is_some() {
        let email = credentials.email.clone().unwrap().clone();
        let email_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::email.eq(email.clone())))).get_result(connection);
        if email_exists.is_err() {
            return Err(StatusResponse::new("Could not search for email in database".parse().unwrap(), false));
        }
        if !email_exists.ok().unwrap() {
            return Err(StatusResponse::new("Email address not found".parse().unwrap(), false));
        }
        get_account_result = accounts::dsl::accounts.filter(accounts::email.eq(email.clone())).first::<Account>(connection);
        if get_account_result.is_err() {
            return Err(StatusResponse::new("Could not get account from database".parse().unwrap(), false));
        }
    } else if credentials.username.is_some() {
        let username = credentials.username.clone().unwrap().clone();
        let username_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::username.eq(username.clone())))).get_result(connection);
        if username_exists.is_err() {
            return Err(StatusResponse::new("Could not search for username in database".parse().unwrap(), false));
        }
        if !username_exists.ok().unwrap() {
            return Err(StatusResponse::new("UsernameNotFoundError".parse().unwrap(), false));
        }
        get_account_result = accounts::dsl::accounts.filter(accounts::username.eq(username.clone())).first::<Account>(connection);
        if get_account_result.is_err() {
            return Err(StatusResponse::new("Could not get account from database".parse().unwrap(), false));
        }
    } else {
        return Err(StatusResponse::new("Both username and password missing".parse().unwrap(), false));
    }
    let account = get_account_result.unwrap();
    if !account.verified {
        return Err(StatusResponse::new("User is not verified".parse().unwrap(), false));
    }
    let access_token = Token::create_access_token(account.id);
    let refresh_token = RefreshToken::create_refresh_token(account.id, account.password_identifier.clone(), connection);
    Ok(TokenResponse::new(account, access_token, refresh_token))
}

/// Gets account by account_id from DB
pub(crate) fn get_account_by_id(id: i32, connection: &PgConnection) -> Account {
    accounts::dsl::accounts.filter(accounts::id.eq(id)).first::<Account>(connection).unwrap()
}

/// Creates random password identifier to check if password has changed
fn create_password_identifier() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(10).collect()
}

/// Creates random verification token for email verification
fn create_verification_token() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(6).collect()
}

/// Registers account using DB
pub(crate) fn create(account: NewAccount, connection: &PgConnection) -> ApiResponse {
    let is_valid: StatusResponse = account.is_valid(connection);
    if !is_valid.status {
        return ApiResponse {
            json: is_valid.to_string(),
            status: Status::BadRequest,
        };
    }
    let hashed_password = hash(account.password.clone(), 10).expect("Error hashing password: ");
    let password_identifier = create_password_identifier();
    let token = create_verification_token();
    let account = NewDBAccount {
        created_at: Utc::now(),
        updated_at: Some(Utc::now()),
        deleted_at: None,
        email: account.email.clone(),
        username: account.username.clone(),
        password: hashed_password,
        image_url: "".to_string(),
        password_identifier: password_identifier.clone(),
        verified: false,
        shared_prefs: "".to_string(),
    };
    let insert_result: Result<Account, Error> = diesel::insert_into(accounts::table)
        .values(&account)
        .returning(accounts::all_columns)
        .get_result(connection);
    if insert_result.is_err() {
        return ApiResponse {
            json: StatusResponse::new("Could not insert account into DB".parse().unwrap(), false).to_string(),
            status: Status::BadRequest,
        };
    }
    let account = insert_result.unwrap();
    let token_db = VerificationToken {
        account_id: account.id,
        token,
        created_at: Local::now().to_rfc3339_opts(SecondsFormat::Millis, true),
    };
    let token_insert_result = diesel::insert_into(tokens::table)
        .values(&token_db)
        .execute(connection);
    if token_insert_result.is_err() {
        return ApiResponse {
            json: StatusResponse::new("Could not insert token into DB".parse().unwrap(), false).to_string(),
            status: Status::BadRequest,
        };
    }
    let domain = env::var("DOMAIN").expect("Could not find DOMAIN in .env");
    let email = create_token_email(account.username.clone(), format!("{}/api/users/verify/{}/{}", domain, token_db.account_id, token_db.token));
    send_email(email, account.email.clone());
    let access_token = Token::create_access_token(account.id);
    let refresh_token = RefreshToken::create_refresh_token(account.id, password_identifier.clone(), connection);
    let json = TokenResponse {
        message: "AccCreationSuccess".parse().unwrap(),
        status: true,
        account: TokenAccount::from_account(account, access_token, refresh_token),
    }.to_string();
    return ApiResponse {
        json,
        status: Status::Ok,
    };
}

/// Verifies email token and verifies user in DB
pub(crate) fn verify_email(account_id: i32, used_token: String, connection: &PgConnection) -> StatusResponse {
    let id_exists: bool = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))).get_result(connection).expect("Could not check if account exists");
    if !id_exists {
        return StatusResponse::new("UserNotFoundError".parse().unwrap(), false);
    }
    let mut account = accounts::dsl::accounts.filter(accounts::id.eq(account_id)).first::<Account>(connection).unwrap();
    let token_exists: bool = select(exists(tokens::dsl::tokens.filter(tokens::account_id.eq(account_id)))).get_result(connection).expect("Could not check if token exists");
    if !token_exists {
        return StatusResponse::new("TokenNotFoundError".parse().unwrap(), false);
    }
    let saved_token = tokens::dsl::tokens.filter(tokens::account_id.eq(account_id)).first::<VerificationToken>(connection).unwrap();
    return if saved_token.token.eq(&used_token) {
        account.verified = true;
        let update_result = diesel::update(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))
            .set(accounts::verified.eq(true))
            .execute(connection);
        if update_result.is_err() {
            return StatusResponse::new("Can not update status of account".parse().unwrap(), false);
        }
        let delete_result = diesel::delete(tokens::dsl::tokens.filter(tokens::account_id.eq(account_id))).execute(connection);
        if delete_result.is_err() {
            return StatusResponse::new("Can not remove token".parse().unwrap(), false);
        }
        StatusResponse::new("VerificationSucces".parse().unwrap(), true)
    } else {
        StatusResponse::new("Token does not match".parse().unwrap(), false)
    };
}

pub(crate) fn change_info(account_id: i32, account: PatchingAccount, connection: &PgConnection) -> ApiResponse {
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return ApiResponse {
            json: StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string(),
            status: Status::BadRequest,
        };
    }
    let mut has_changes = false;
    if account.shared_prefs.is_some() {
        has_changes = true;
        let change_result = change_shared_prefs(account_id, account.shared_prefs.unwrap(), connection);
        if change_result.is_err() {
            return change_result.err().unwrap()
        }
    }
    if account.username.is_some() {
        has_changes = true;
        let change_result = change_username(account_id, account.username.unwrap(), connection);
        if change_result.is_err() {
            return change_result.err().unwrap()
        }
    }
    if account.password.is_some() {
        has_changes = true;
        let change_result = change_password(account_id, account.password.unwrap(), connection);
        if change_result.is_err() {
            return change_result.err().unwrap()
        }
    }
    if account.image_url.is_some() {
        has_changes = true;
        let change_result = change_image_url(account_id, account.image_url.unwrap(), connection);
        if change_result.is_err() {
            return change_result.err().unwrap()
        }
    }
    return if has_changes {
        ApiResponse {
            json: StatusResponse::new("UpdateSuccess".parse().unwrap(), true).to_string(),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            json: StatusResponse::new("No changes".parse().unwrap(), false).to_string(),
            status: Status::BadRequest,
        }
    }
}

/// Changes shared preferences of user
pub(crate) fn change_shared_prefs(account_id: i32, new_shared_prefs: String, connection: &PgConnection) -> Result<(), ApiResponse> {
    let update_result = diesel::update(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))
        .set(accounts::shared_prefs.eq(new_shared_prefs))
        .execute(connection);
    return if update_result.is_err() {
        Err(ApiResponse {
            json: StatusResponse::new(update_result.err().unwrap().to_string(), false).to_string(),
            status: Status::BadRequest,
        })
    } else {
        Ok(())
    };
}

/// Changes username of user
pub(crate) fn change_username(account_id: i32, new_username: String, connection: &PgConnection) -> Result<(), ApiResponse> {
    let username_valid = validate_username(new_username.clone(), connection);
    if username_valid.is_err() {
        return Err(ApiResponse {
            json: username_valid.err().unwrap().to_string(),
            status: Status::BadRequest,
        });
    }
    let update_result = diesel::update(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))
        .set(accounts::username.eq(new_username))
        .execute(connection);
    return if update_result.is_err() {
        Err(ApiResponse {
            json: StatusResponse::new(update_result.err().unwrap().to_string(), false).to_string(),
            status: Status::BadRequest,
        })
    } else {
        Ok(())
    };
}

/// Changes password of user
pub(crate) fn change_password(account_id: i32, new_password: String, connection: &PgConnection) -> Result<(), ApiResponse> {
    let password_valid = validate_password(new_password.clone());
    if password_valid.is_err() {
        return Err(ApiResponse {
            json: password_valid.err().unwrap().to_string(),
            status: Status::BadRequest,
        });
    }
    let password_identifier = create_password_identifier();
    let hashed_password = hash(new_password.clone(), 10).unwrap();
    let update_result = diesel::update(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))
        .set((accounts::password.eq(hashed_password),
              accounts::password_identifier.eq(password_identifier)))
        .execute(connection);
    return if update_result.is_err() {
        Err(ApiResponse {
            json: StatusResponse::new(update_result.err().unwrap().to_string(), false).to_string(),
            status: Status::BadRequest,
        })
    } else {
        Ok(())
    };
}

/// Changes image url of user
pub(crate) fn change_image_url(account_id: i32, new_image_url: String, connection: &PgConnection) -> Result<(), ApiResponse> {
    let update_result = diesel::update(accounts::dsl::accounts.filter(accounts::id.eq(account_id)))
        .set(accounts::image_url.eq(new_image_url))
        .execute(connection);
    return if update_result.is_err() {
        Err(ApiResponse {
            json: StatusResponse::new(update_result.err().unwrap().to_string(), false).to_string(),
            status: Status::BadRequest,
        })
    } else {
        Ok(())
    };
}


/// Gets info of user identified by account id
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