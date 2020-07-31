use diesel::PgConnection;
use crate::responders::ApiResponse;
use crate::setting::responses::{INVALID_KEY, SETTING_UPDATED, SETTING_DOESNT_EXIST};
use crate::setting::repository::{update_or_insert_setting, setting_exists, get_setting_if_exists, get_all_settings};
use crate::setting::model::Setting;
use chrono::{Utc, TimeZone};
use crate::responses::INTERNAL_DB_ERROR;

/// Checks if the key is in snake_case.
fn is_valid_key(key: &str) -> bool {
    key.chars().all(|c| {
        (c >= 'a' && c <= 'z')
            || c == '_'
    })
}

/// Updates or insert a setting based on its key and value.
pub(crate) fn change_setting(key: String, value: String, account_id: String, connection: &PgConnection) -> ApiResponse {
    if !is_valid_key(&*key) {
        return INVALID_KEY;
    }

    let setting = Setting {
        setting_key: key,
        account_id,
        setting_value: value,
        last_modify_date: Utc::now(),
    };
    return match update_or_insert_setting(setting, connection) {
        Err(error) => {
            println!("Unable to update or insert setting: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_rows_updated) => {
            SETTING_UPDATED
        }
    };
}

/// Gets value of a setting based on its key.
/// When the value is never set an error is returned.
pub(crate) fn get_setting(key: String, account_id: String, connection: &PgConnection) -> Result<String, ApiResponse> {
    if !is_valid_key(&*key) {
        return Err(INVALID_KEY);
    }

    if !setting_exists(&key, &account_id, &connection) {
        return Err(SETTING_DOESNT_EXIST);
    }

    return match get_setting_if_exists(&key, &account_id, connection) {
        Err(error) => {
            println!("Error getting setting: {}", error);
            Err(INTERNAL_DB_ERROR)
        },
        Ok(value) => {
            Ok(value)
        }
    }
}

pub(crate) fn get_changed_settings(last_updated: i64, account_id: String, connection: &PgConnection) -> Result<String, ApiResponse> {
    return match get_all_settings(&account_id, &connection) {
        Err(error) => {
            println!("Error getting changed settings: {}", error);
            Err(INTERNAL_DB_ERROR)
        },
        Ok(settings) => {
            let changed_settings: Vec<(String, String)> = settings.into_iter()
                .filter(|setting| setting.last_modify_date.timestamp() >= last_updated)
                .map(|setting| (setting.setting_key, setting.setting_value))
                .collect();
            Ok(serde_json::to_string(&changed_settings).unwrap())
        }
    }

}