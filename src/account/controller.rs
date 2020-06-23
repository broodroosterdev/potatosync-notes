use std::env;

use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, select};
use diesel::expression::exists::exists;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::result::Error;
use reqwest::blocking::*;
use rocket_failure::errors::Status;
use uuid::Uuid;

use crate::account::model::{Account, InfoResponse, LoginCredentials, NewAccount, password_is_valid, PatchingAccount, TokenAccount, TokenResponse, username_is_valid, VerificationToken};
use crate::account::repository::{account_from_email, account_from_username, email_exists, insert_account, insert_verification_token, username_exists};
use crate::account::responses::{INCORRECT_CREDENTIALS, INTERNAL_ERROR, INVALID_EMAIL, INVALID_PASSWORD, INVALID_USERNAME, USER_NOT_VERIFIED};
use crate::responses::{ApiError, TokenSuccess};
use crate::schema::accounts;
use crate::schema::notes;
use crate::schema::verification_tokens;
use crate::status_response::{ApiResponse, StatusResponse};

#[derive(Deserialize, Serialize)]
pub struct LoginResponse{
    access_token: &'static str,
    expires_in: u64,
    refresh_expires_in: u64,
    refresh_token: &'static str,
    token_type: &'static str,
    id_token: &'static str,
    #[serde(alias = "not-before-policy")]
    not_before_policy: u64,
    session_statie: &'static str,
    scope: &'static str,
}
/*
/// Used to login user using DB and returns Error if credentials are incorrect
pub(crate) fn login(credentials: LoginCredentials, connection: &PgConnection) -> Result<TokenSuccess, ApiError> {
    let parameters = [
        ("client_id", "notes-api"),
        ("grant_type", "password"),
        ("username", credentials.username.as_str()),
        ("password", credentials.password.as_str()),
        ("scope", "openid")
    ];
    let client = Client::new();
    let response = client.post("http://localhost:8080/auth/realms/potatosync/protocol/openid-connect/token").form(&parameters).send().unwrap();
    if response.status().is_success(){
        let login_response: LoginResponse = response.json().unwrap();
        Ok(TokenSuccess{
            description: "",
            code: 0,
            access_token: login_response.access_token,
            refresh_token: login_response.refresh_token,
            id_token: login_response.id_token
        })
    } else {
        Err(INCORRECT_CREDENTIALS)
    }
}*/
/*
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
    let validation_result = account.is_valid(connection);
    if validation_result.is_err(){
        println!("Invalid");
        return validation_result.err().unwrap().to_response();
    }

    match email_exists(&account.email, connection) {
        Err(error) => {
            println!("exists");
            return INVALID_EMAIL.to_response();
        },
        _ => {}
    }
    match username_exists(&account.username, connection) {
        Err(error) => {
            println!("exists");
            return INVALID_USERNAME.to_response();
        },
        _ => {}
    }
    let hashed_password = hash(account.password.clone(), 12).expect("Error hashing password: ");
    let password_identifier = create_password_identifier();
    let verification_token = create_verification_token();
    let mut account = Account {
        id: Uuid::new_v4().to_string(),
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
    if env::var("DISABLE_EMAIL_VERIFICATION").is_ok() && env::var("DISABLE_EMAIL_VERIFICATION").unwrap() == "true" {
        println!("IMPORTANT: Automatically verifying user. If you did not intend this behaviour please kill the program and check your settings");
        account.verified = true;
    }
    return match insert_account(&account, connection) {
        Err(_) => INTERNAL_ERROR.to_response(),
        Ok(account) => {
            if env::var("DISABLE_EMAIL_VERIFICATION").is_err() || env::var("DISABLE_EMAIL_VERIFICATION").unwrap() != "true" {
                let token_db = VerificationToken {
                    account_id: account.id.clone(),
                    verification_token,
                    expires_at: Utc::now() + Duration::days(1),
                };
                match insert_verification_token(&token_db, connection) {
                    Err(error) => {
                        println!("Could not save verification token in database: {}", error);
                        return INTERNAL_ERROR.to_response()
                    },
                    _ => {}
                }
                let domain = env::var("DOMAIN").expect("Could not find DOMAIN in .env");
                let email = create_verification_email(account.username.clone(), format!("{}/api/users/verify/{}/{}", domain, token_db.account_id, token_db.verification_token));
                send_email(email, "Confirming your new Potatosync account", account.email.clone());
            }
            let json = TokenResponse {
                message: "AccCreationSuccess".parse().unwrap(),
                status: true,
                account: TokenAccount::from_account(account, None, None),
            }.to_string();
            ApiResponse {
                json,
                status: Status::Ok,
            }
        }
    }

}

/// Verifies email token and verifies user in DB
pub(crate) fn verify_email(account_id: String, used_token: String, connection: &PgConnection) -> StatusResponse {
    let id_exists: bool = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(&account_id)))).get_result(connection).expect("Could not check if account exists");
    if !id_exists {
        return StatusResponse::new("UserNotFoundError".parse().unwrap(), false);
    }
    let mut account = accounts::dsl::accounts.filter(accounts::id.eq(&account_id)).first::<Account>(connection).unwrap();
    let token_exists: bool = select(exists(verification_tokens::dsl::verification_tokens.filter(verification_tokens::account_id.eq(&account_id)))).get_result(connection).expect("Could not check if token exists");
    if !token_exists {
        return StatusResponse::new("TokenNotFoundError".parse().unwrap(), false);
    }
    let saved_token = verification_tokens::dsl::verification_tokens.filter(verification_tokens::account_id.eq(&account_id)).first::<VerificationToken>(connection).unwrap();
    return if saved_token.verification_token.eq(&used_token) {
        account.verified = true;
        let update_result = diesel::update(accounts::dsl::accounts.filter(accounts::id.eq(&account_id)))
            .set(accounts::verified.eq(true))
            .execute(connection);
        if update_result.is_err() {
            return StatusResponse::new("Can not update status of account".parse().unwrap(), false);
        }
        let delete_result = diesel::delete(verification_tokens::dsl::verification_tokens.filter(verification_tokens::account_id.eq(&account_id))).execute(connection);
        if delete_result.is_err() {
            return StatusResponse::new("Can not remove token".parse().unwrap(), false);
        }
        StatusResponse::new("VerificationSucces".parse().unwrap(), true)
    } else {
        StatusResponse::new("Token does not match".parse().unwrap(), false)
    };
}

pub(crate) fn change_info(account_id: String, account: PatchingAccount, connection: &PgConnection) -> ApiResponse {
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(&account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return ApiResponse {
            json: StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string(),
            status: Status::BadRequest,
        };
    }
    let mut has_changes = false;
    if account.shared_prefs.is_some() {
        has_changes = true;
        let change_result = change_shared_prefs(account_id.clone(), account.shared_prefs.unwrap(), connection);
        if change_result.is_err() {
            return change_result.err().unwrap();
        }
    }
    if account.username.is_some() {
        has_changes = true;
        let change_result = change_username(account_id.clone(), account.username.unwrap(), connection);
        if change_result.is_err() {
            return change_result.err().unwrap();
        }
    }
    if account.password.is_some() {
        has_changes = true;
        let change_result = change_password(account_id.clone(), account.password.unwrap(), connection);
        if change_result.is_err() {
            return change_result.err().unwrap();
        }
    }
    if account.image_url.is_some() {
        has_changes = true;
        let change_result = change_image_url(account_id.clone(), account.image_url.unwrap(), connection);
        if change_result.is_err() {
            return change_result.err().unwrap();
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
    };
}

/// Changes shared preferences of user
pub(crate) fn change_shared_prefs(account_id: String, new_shared_prefs: String, connection: &PgConnection) -> Result<(), ApiResponse> {
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
pub(crate) fn change_username(account_id: String, new_username: String, connection: &PgConnection) -> Result<(), ApiResponse> {
    match username_exists(&new_username, connection){
        Err(error) => {
            println!("Could not check if username exists: {}", error);
            return Err(INVALID_USERNAME.to_response())
        }
        _ => {}
    }
    match username_is_valid(&new_username){
        false => {
            return Err(INVALID_USERNAME.to_response())
        }
        _ => {}
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
pub(crate) fn change_password(account_id: String, new_password: String, connection: &PgConnection) -> Result<(), ApiResponse> {
    let password_valid = password_is_valid(&new_password);
    if !password_valid {
        return Err(INVALID_PASSWORD.to_response());
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
pub(crate) fn change_image_url(account_id: String, new_image_url: String, connection: &PgConnection) -> Result<(), ApiResponse> {
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
pub(crate) fn get_info(account_id: String, connection: &PgConnection) -> String {
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(&account_id)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string();
    }
    let account = accounts::dsl::accounts.filter(accounts::id.eq(&account_id)).first::<Account>(connection).unwrap();
    return InfoResponse::new(account).to_string();
}

/// Delete user
pub(crate) fn delete_user(refresh_token: RefreshToken, connection: &PgConnection) -> ApiResponse {
    let id_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::id.eq(&refresh_token.sub)))).get_result(connection);
    if !id_exists.ok().unwrap() {
        return ApiResponse {
            json: StatusResponse::new("UserNotFoundError".parse().unwrap(), false).to_string(),
            status: Status::NotFound,
        };
    }
    let delete_result = diesel::delete(notes::table)
        .filter(notes::dsl::account_id.eq(&refresh_token.sub))
        .execute(connection);
    return if delete_result.is_err() {
        ApiResponse {
            json: StatusResponse::new("Error while removing account".parse().unwrap(), false).to_string(),
            status: Status::BadRequest,
        }
    } else {
        ApiResponse {
            json: StatusResponse::new("DeletionSuccess".parse().unwrap(), true).to_string(),
            status: Status::Ok,
        }
    };
}

#[cfg(test)]
mod tests {
    use bcrypt::DEFAULT_COST;
    use mocktopus::mocking::*;

    use crate::db;

    use super::*;

    #[test]
    fn success_response_when_login_successful() {
        dotenv::dotenv().ok();
        email_exists.mock_safe(|_, _| MockResult::Return(Ok(true)));
        let mut account = Account::mock_empty();
        account.password = hash("test", DEFAULT_COST).unwrap();
        account.verified = true;
        account_from_email.mock_safe(move |_, _| MockResult::Return(Ok(account.clone())));
        let credentials = LoginCredentials {
            email: Some("test@example.com".to_string()),
            username: None,
            password: "test".to_string(),
        };
        let login_result = login(credentials, &db::connect().get().unwrap());
        assert!(login_result.is_ok());
        println!("{}", login_result.ok().unwrap().to_string());
    }

    #[test]
    fn invalid_on_missing_username_and_password_on_login() {
        dotenv::dotenv().ok();
        let credentials = LoginCredentials {
            email: None,
            username: None,
            password: "".to_string(),
        };
        let login_result = login(credentials, &db::connect().get().unwrap());
        let expected_result = INCORRECT_CREDENTIALS;
        assert!(login_result.is_err());
        let login_result = login_result.err().unwrap();
        println!("{}", login_result.to_string());
        assert_eq!(login_result.code, expected_result.code);
        assert_eq!(login_result.description, expected_result.description);
    }

    #[test]
    fn invalid_on_non_existing_email_on_login() {
        dotenv::dotenv().ok();
        email_exists.mock_safe(|_, _| MockResult::Return(Ok(false)));
        let credentials = LoginCredentials {
            email: Some("test@example.com".to_string()),
            username: None,
            password: "".to_string(),
        };
        let login_result = login(credentials, &db::connect().get().unwrap());
        let expected_result = INCORRECT_CREDENTIALS;
        assert!(login_result.is_err());
        let login_result = login_result.err().unwrap();
        println!("{}", login_result.to_string());
        assert_eq!(login_result.code, expected_result.code);
        assert_eq!(login_result.description, expected_result.description);
    }

    #[test]
    fn invalid_on_non_existing_username_on_login() {
        dotenv::dotenv().ok();
        username_exists.mock_safe(|_, _| MockResult::Return(Ok(false)));
        let credentials = LoginCredentials {
            email: None,
            username: Some("test".to_string()),
            password: "".to_string(),
        };
        let login_result = login(credentials, &db::connect().get().unwrap());
        let expected_result = INCORRECT_CREDENTIALS;
        assert!(login_result.is_err());
        let login_result = login_result.err().unwrap();
        println!("{}", login_result.to_string());
        assert_eq!(login_result.code, expected_result.code);
        assert_eq!(login_result.description, expected_result.description);
    }

    #[test]
    fn invalid_when_error_account_from_email_on_login() {
        dotenv::dotenv().ok();
        email_exists.mock_safe(|_, _| MockResult::Return(Ok(true)));
        account_from_email.mock_safe(|_, _| MockResult::Return(Err(String::from("Error"))));
        let credentials = LoginCredentials {
            email: Some("test@example.com".to_string()),
            username: None,
            password: "".to_string(),
        };
        let login_result = login(credentials, &db::connect().get().unwrap());
        let expected_result = INCORRECT_CREDENTIALS;
        assert!(login_result.is_err());
        let login_result = login_result.err().unwrap();
        println!("{}", login_result.to_string());
        assert_eq!(login_result.code, expected_result.code);
        assert_eq!(login_result.description, expected_result.description);
    }

    #[test]
    fn invalid_when_error_account_from_username_on_login() {
        dotenv::dotenv().ok();
        username_exists.mock_safe(|_, _| MockResult::Return(Ok(true)));
        account_from_username.mock_safe(|_, _| MockResult::Return(Err(String::from("Error"))));
        let credentials = LoginCredentials {
            email: None,
            username: Some("test".to_string()),
            password: "".to_string(),
        };
        let login_result = login(credentials, &db::connect().get().unwrap());
        let expected_result = INCORRECT_CREDENTIALS;
        assert!(login_result.is_err());
        let login_result = login_result.err().unwrap();
        println!("{}", login_result.to_string());
        assert_eq!(login_result.code, expected_result.code);
        assert_eq!(login_result.description, expected_result.description);
    }

    #[test]
    fn invalid_when_password_not_matching() {
        dotenv::dotenv().ok();
        email_exists.mock_safe(|_, _| MockResult::Return(Ok(true)));
        let mut account = Account::mock_empty();
        account.password = hash("test", DEFAULT_COST).unwrap();
        account_from_email.mock_safe(move |_, _| MockResult::Return(Ok(account.clone())));
        let credentials = LoginCredentials {
            email: Some("test@example.com".to_string()),
            username: None,
            password: "not test".to_string(),
        };
        let login_result = login(credentials, &db::connect().get().unwrap());
        let expected_result = INCORRECT_CREDENTIALS;
        assert!(login_result.is_err());
        let login_result = login_result.err().unwrap();
        println!("{}", login_result.to_string());
        assert_eq!(login_result.code, expected_result.code);
        assert_eq!(login_result.description, expected_result.description);
    }

    #[test]
    fn not_verified_error_when_not_verified_on_login() {
        dotenv::dotenv().ok();
        email_exists.mock_safe(|_, _| MockResult::Return(Ok(true)));
        let mut account = Account::mock_empty();
        account.password = hash("test", DEFAULT_COST).unwrap();
        account_from_email.mock_safe(move |_, _| MockResult::Return(Ok(account.clone())));
        let credentials = LoginCredentials {
            email: Some("test@example.com".to_string()),
            username: None,
            password: "not test".to_string(),
        };
        let login_result = login(credentials, &db::connect().get().unwrap());
        let expected_result = INCORRECT_CREDENTIALS;
        assert!(login_result.is_err());
        let login_result = login_result.err().unwrap();
        println!("{}", login_result);
        assert_eq!(login_result.description, expected_result.description);
        assert_eq!(login_result.code, expected_result.code);
    }

    #[test]
    fn success_response_when_create_successful() {
        dotenv::dotenv().ok();
        env::set_var("DISABLE_EMAIL_VERIFICATION", "true");
        email_exists.mock_safe(|_, _| MockResult::Return(Ok(false)));
        username_exists.mock_safe(|_, _| MockResult::Return(Ok(false)));
        let account = Account {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            deleted_at: None,
            email: "test@example.com".to_string(),
            username: "test123".to_string(),
            password: hash("strongEnoughPassword".to_string(), 12).unwrap(),
            image_url: "".to_string(),
            password_identifier: create_password_identifier(),
            verified: false,
            shared_prefs: "".to_string(),
        };
        let account_copy = account.clone();
        insert_account.mock_safe(move |_, _| MockResult::Return(Ok(account_copy.clone())));
        let new_account = NewAccount {
            email: "test@example.com".to_string(),
            username: "test123".to_string(),
            password: "strongEnoughPassword".to_string(),
        };
        let create_result = create(new_account, &db::connect().get().unwrap());
        let expected_result = ApiResponse {
            json: TokenResponse {
                message: "AccCreationSuccess".to_string(),
                status: true,
                account: TokenAccount::from_account(account.clone(), None, None),
            }.to_string(),
            status: Status::Ok,
        };
        println!("{} : {}", create_result.status, create_result.json);
        assert_eq!(create_result.status, expected_result.status);
        assert_eq!(create_result.json, expected_result.json);
    }
}
 */