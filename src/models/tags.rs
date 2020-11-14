use chrono::{DateTime, Utc};
use crate::schema::tags;
use chrono::serde::ts_milliseconds::*;

#[derive(Queryable, Serialize, Deserialize, Insertable, PartialEq, Clone)]
#[table_name = "tags"]
pub struct Tag {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub color: i32,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub last_modify_date: DateTime<Utc>,
}