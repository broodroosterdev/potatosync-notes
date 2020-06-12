use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, select};
use diesel::expression::exists::exists;
use diesel::query_dsl::filter_dsl::FilterDsl;
#[cfg(test)]
use mocktopus::macros::*;

use crate::account::model::{Account, VerificationToken};
use crate::schema::accounts;
use crate::schema::verification_tokens;

/// Gets account by account_id from DB
#[cfg_attr(test, mockable)]
pub(crate) fn get_account_by_id(id: String, connection: &PgConnection) -> Account {
    accounts::dsl::accounts.filter(accounts::id.eq(id)).first::<Account>(connection).unwrap()
}

#[cfg_attr(test, mockable)]
pub fn email_exists(email: &String, connection: &PgConnection) -> Result<bool, String> {
    let email_exists: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts
        .filter(accounts::email.eq(email))))
        .get_result(connection);
    return match email_exists {
        Err(error) => Err(error.to_string()),
        Ok(exists) => Ok(exists)
    }
}

#[cfg_attr(test, mockable)]
pub fn username_exists(username: &String, connection: &PgConnection) -> Result<bool, String> {
    let exists_result: Result<bool, diesel::result::Error> = select(exists(accounts::dsl::accounts.filter(accounts::username.eq(username.clone())))).get_result(connection);
    return match exists_result {
        Err(error) => Err(error.to_string()),
        Ok(exists) => Ok(exists)
    }
}

#[cfg_attr(test, mockable)]
pub fn account_from_email(email: &String, connection: &PgConnection) -> Result<Account, String> {
    let filter_result = accounts::dsl::accounts.filter(accounts::email.eq(email)).first::<Account>(connection);
    return match filter_result {
        Err(error) => Err(error.to_string()),
        Ok(account) => Ok(account)
    }
}

#[cfg_attr(test, mockable)]
pub fn account_from_username(username: &String, connection: &PgConnection) -> Result<Account, String> {
    let filter_result = accounts::dsl::accounts.filter(accounts::username.eq(username.clone())).first::<Account>(connection);
    return match filter_result {
        Err(error) => Err(error.to_string()),
        Ok(account) => Ok(account)
    }
}

#[cfg_attr(test, mockable)]
pub fn insert_account(account: &Account, connection: &PgConnection) -> Result<Account, String>{
    let insert_result = diesel::insert_into(accounts::table)
        .values(account)
        .returning(accounts::all_columns)
        .get_result(connection);
    return match insert_result {
        Err(error) => Err(error.to_string()),
        Ok(account) => Ok(account)
    }
}

pub fn insert_verification_token(token: &VerificationToken, connection: &PgConnection) -> Result<usize, String>{
    let insert_result = diesel::insert_into(verification_tokens::table)
        .values(token)
        .execute(connection);
    return match insert_result {
        Err(error) => Err(error.to_string()),
        Ok(changed) => Ok(changed)
    };
}