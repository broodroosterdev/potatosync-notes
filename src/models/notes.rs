use chrono::{Utc, DateTime};
use crate::schema::notes;
use chrono::serde::ts_milliseconds::*;

/// General Note struct used for retrieving from db and updating notes
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset, PartialEq, Clone)]
#[table_name = "notes"]
#[primary_key(note_id, account_id)]
#[changeset_options(treat_none_as_null = "true")]
// {"note_id": 1, "title": "Test", "content": "Test", "is_starred": false, "date": "2020-03-19T14:21:06.275Z", "color": 0, "image_url":null, "is_list": false, "list_parse_string":null, "reminders":null, "hide_content": false, "pin":null, "password":null, "is_deleted": false, "is_archived": false}
pub struct Note {
    pub note_id: String,
    pub account_id: String,
    pub title: String,
    pub content: String,
    pub style_json: String,
    pub starred: bool,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub creation_date: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub last_modify_date: DateTime<Utc>,
    pub color: i32,
    pub images: String,
    pub list: bool,
    pub list_content: String,
    pub reminders: String,
    pub tags: String,
    pub hide_content: bool,
    pub lock_note: bool,
    pub uses_biometrics: bool,
    pub deleted: bool,
    pub archived: bool,
}

#[cfg(test)]
impl Note {
    pub(crate) fn mock_empty() -> Note {
        Note {
            note_id: "".to_string(),
            account_id: "".to_string(),
            title: "".to_string(),
            content: "".to_string(),
            style_json: "".to_string(),
            starred: false,
            creation_date: Utc::now(),
            last_modify_date: Utc::now(),
            color: 0,
            images: "".to_string(),
            list: false,
            list_content: "".to_string(),
            reminders: "".to_string(),
            tags: "".to_string(),
            hide_content: false,
            lock_note: false,
            uses_biometrics: false,
            deleted: false,
            archived: false,
        }
    }
}