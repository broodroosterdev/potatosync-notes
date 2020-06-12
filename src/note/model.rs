use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds::*;
use serde_derive::*;

use crate::account::model::{deserialize_option, serialize_option};
use crate::schema::notes;

/// General Note struct used for retrieving from db and updating notes
#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset, PartialEq, Clone)]
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
            hide_content: false,
            lock_note: false,
            uses_biometrics: false,
            deleted: false,
            archived: false,
        }
    }
}

/// Note as provided by the client when saving. Note the missing account_id since the client doesnt know the id.
#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
pub struct SavingNote {
    pub note_id: String,
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
    pub hide_content: bool,
    pub lock_note: bool,
    pub uses_biometrics: bool,
    pub deleted: bool,
    pub archived: bool,
}

impl SavingNote {
    /// Convert to Note by adding account_id
    pub fn to_note(&self, account_id: String) -> Note {
        Note {
            note_id: self.note_id.clone(),
            account_id,
            title: self.title.clone(),
            content: self.content.clone(),
            style_json: self.style_json.clone(),
            starred: self.starred,
            creation_date: self.creation_date.clone(),
            last_modify_date: self.last_modify_date.clone(),
            color: self.color,
            images: self.images.clone(),
            list: self.list,
            list_content: self.list_content.clone(),
            reminders: self.reminders.clone(),
            hide_content: self.hide_content,
            lock_note: self.lock_note,
            uses_biometrics: self.uses_biometrics,
            deleted: self.deleted,
            archived: self.archived,
        }
    }
}

#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset, PartialEq, Clone)]
pub struct PatchingNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub style_json: Option<String>,
    pub starred: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option")]
    #[serde(serialize_with = "serialize_option")]
    pub last_modify_date: Option<DateTime<Utc>>,
    pub color: Option<i32>,
    pub images: Option<String>,
    pub list: Option<bool>,
    pub list_content: Option<String>,
    pub reminders: Option<String>,
    pub hide_content: Option<bool>,
    pub lock_note: Option<bool>,
    pub uses_biometrics: Option<bool>,
    pub deleted: Option<bool>,
    pub archived: Option<bool>,
}

#[cfg(test)]
impl PatchingNote {
    pub(crate) fn mock_empty() -> PatchingNote {
        return PatchingNote {
            title: None,
            content: None,
            style_json: None,
            starred: None,
            last_modify_date: None,
            color: None,
            images: None,
            list: None,
            list_content: None,
            reminders: None,
            hide_content: None,
            lock_note: None,
            uses_biometrics: None,
            deleted: None,
            archived: None,
        }
    }
    pub(crate) fn mock_empty_with_last_modify() -> PatchingNote {
        return PatchingNote {
            title: None,
            content: None,
            style_json: None,
            starred: None,
            last_modify_date: Some(Utc::now()),
            color: None,
            images: None,
            list: None,
            list_content: None,
            reminders: None,
            hide_content: None,
            lock_note: None,
            uses_biometrics: None,
            deleted: None,
            archived: None,
        }
    }
}

/// Struct used when client needs to specify certain note_id
#[derive(Serialize, Deserialize)]
pub struct NoteId {
    pub(crate) note_id: i32,
}

/// Struct used when client needs to specify last_updated timestamp
#[derive(Serialize, Deserialize)]
pub struct NoteLastUpdated {
    pub(crate) last_updated: String
}

/// Struct used when server needs to return list of notes along with status of request
#[derive(Serialize, Deserialize)]
pub struct NoteResponse {
    pub(crate) message: String,
    pub(crate) status: bool,
    pub(crate) notes: Vec<Note>,
}

impl ToString for NoteResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
