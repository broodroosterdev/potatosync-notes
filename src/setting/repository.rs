use diesel::prelude::*;
#[cfg(test)]
use mocktopus::macros::*;

use crate::schema::settings;
use crate::setting::model::Setting;

#[cfg_attr(test, mockable)]
pub fn update_or_insert_setting(setting: Setting, connection: &PgConnection) -> Result<usize, String>{
    let insert_result = diesel::insert_into(settings::table)
        .values(&setting)
        .on_conflict((settings::setting_key, settings::account_id))
        .do_update()
        .set(&setting)
        .execute(connection);
    return match insert_result {
        Err(error) => Err(error.to_string()),
        Ok(rows_updated) => Ok(rows_updated)
    }
}

#[cfg_attr(test, mockable)]
pub fn setting_exists(key: &String, account_id: &String, connection: &PgConnection) -> bool {
    let setting_exists = settings::dsl::settings.select(settings::setting_key)
        .filter(settings::setting_key.eq(key))
        .filter(settings::account_id.eq(account_id))
        .first::<String>(connection);
    return setting_exists.is_ok();
}

#[cfg_attr(test, mockable)]
pub fn get_setting_if_exists(key: &String, account_id: &String, connection: &PgConnection) -> Result<String, String> {
    let get_result = settings::dsl::settings.select(settings::setting_value)
        .find((key, account_id))
        .first(connection);
    return match get_result{
        Err(error) => Err(error.to_string()),
        Ok(existing_list) => Ok(existing_list)
    }
}