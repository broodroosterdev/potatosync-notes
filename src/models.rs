use serde::{Deserialize, Serialize};
use crate::schema::notes;
use crate::schema::tags;

#[derive(Deserialize, Insertable, Queryable, Serialize)]
pub struct Note {
    pub id: String,
    #[serde(skip_serializing)]
    pub account_id: String,
    pub content: String,
    pub last_changed: i64
}

#[derive(Deserialize, Insertable, Queryable, Serialize)]
pub struct Tag {
    pub id: String,
    #[serde(skip_serializing)]
    pub account_id: String,
    pub content: String,
    pub last_changed: i64
}