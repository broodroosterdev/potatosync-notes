use chrono::{DateTime, Utc};
use crate::schema::settings;

/// General Setting struct used for retrieving from db and updating settings
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Insertable, PartialEq, Clone)]
#[table_name = "settings"]
pub struct Setting {
    pub setting_key: String,
    pub account_id: String,
    pub setting_value: String,
    pub last_modify_date: DateTime<Utc>
}
