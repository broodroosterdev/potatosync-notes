use rocket::Data;
use rocket::Rocket;
use crate::auth::claims::Token;
use crate::db::guard::Connection;
use std::io::Read;
use crate::responders::ApiResponse;
use rocket::http::Status;
use crate::models::settings::Setting;
use chrono::Utc;
use crate::crud::settings::{update_or_insert_setting, setting_exists, get_setting_if_exists, get_all_settings};
use std::collections::HashMap;
use rocket_contrib::json::Json;

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/setting", routes![
       change_setting,
       get_setting,
       get_changed_settings
    ])
}

/// Checks if the key is in snake_case.
fn is_valid_key(key: &str) -> bool {
    key.chars().all(|c| {
        (c >= 'a' && c <= 'z')
            || c == '_'
    })
}

// Route for changing setting
#[put("/<key>", data = "<value>")]
fn change_setting(key: String, value: Data, token: Token, connection: Connection) -> ApiResponse {
    let mut value_buffer = String::new();
    value.open().read_to_string(&mut value_buffer).unwrap();
    if !is_valid_key(&*key) {
        return INVALID_KEY;
    }

    let setting = Setting {
        setting_key: key,
        account_id: token.sub,
        setting_value: value_buffer,
        last_modify_date: Utc::now(),
    };
    return match update_or_insert_setting(setting, &*connection) {
        Err(error) => {
            println!("Unable to update or insert setting: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_rows_updated) => {
            SETTING_UPDATED
        }
    };
}

/// Route for getting setting
#[get("/<key>")]
fn get_setting(key: String, token: Token, connection: Connection) -> Result<String, ApiResponse> {
    if !is_valid_key(&*key) {
        return Err(INVALID_KEY);
    }

    if !setting_exists(&key, &token.sub, &connection) {
        return Ok("null".to_string());
    }

    return match get_setting_if_exists(&key, &token.sub, &connection) {
        Err(error) => {
            println!("Error getting setting: {}", error);
            Err(INTERNAL_DB_ERROR)
        },
        Ok(value) => {
            Ok(value)
        }
    }
}

/// Route for getting changed key-value pairs of settings
#[get("/changed?<last_updated>")]
fn get_changed_settings(last_updated: u64, token: Token, connection: Connection) -> Result<Json<HashMap<String, String>>, ApiResponse> {
    return match get_all_settings(&token.sub, &connection) {
        Err(error) => {
            println!("Error getting changed settings: {}", error);
            Err(INTERNAL_DB_ERROR)
        },
        Ok(settings) => {
            let changed_settings: HashMap<String, String> = settings.into_iter()
                .filter(|setting| setting.last_modify_date.timestamp() >= last_updated as i64)
                .map(|setting| (setting.setting_key, setting.setting_value))
                .collect();
            Ok(Json(changed_settings))
        }
    }
}

pub(crate) const INVALID_KEY: ApiResponse = ApiResponse {
    body: "InvalidKey",
    status: Status::BadRequest
};
pub(crate) const SETTING_DOESNT_EXIST: ApiResponse = ApiResponse {
    body: "SettingDoesntExist",
    status: Status::NotFound
};

pub(crate) const SETTING_UPDATED: ApiResponse = ApiResponse {
    body: "SettingUpdated",
    status: Status::Ok
};
pub(crate) const INTERNAL_DB_ERROR: ApiResponse = ApiResponse {
    body: "InternalDatabaseError",
    status: Status::InternalServerError
};