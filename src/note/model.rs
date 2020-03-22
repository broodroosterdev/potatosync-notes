use chrono::{FixedOffset, Local};
use diesel::prelude::*;
use serde_derive::*;

use crate::response::StatusResponse;
use crate::schema::notes;

#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[primary_key(note_id, account_id)]
// {"note_id": 1, "title": "Test", "content": "Test", "is_starred": false, "date": "2020-03-19T14:21:06.275Z", "color": 0, "image_url":null, "is_list": false, "list_parse_string":null, "reminders":null, "hide_content": false, "pin":null, "password":null, "is_deleted": false, "is_archived": false}
pub struct Note {
    pub note_id: i32,
    pub account_id: i32,
    pub title: String,
    pub content: String,
    pub image_url: Option<String>,
    pub list_parse_string: Option<String>,
    pub reminders: Option<String>,
    pub date: String,
    pub color: i32,
    pub hide_content: bool,
    pub is_deleted: bool,
    pub is_archived: bool,
    pub is_list: bool,
    pub is_starred: bool,
    pub pin: Option<String>,
    pub password: Option<String>,
}

#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct NewNote {
    pub account_id: i32,
    pub title: String,
    pub content: String,
    pub image_url: Option<String>,
    pub list_parse_string: Option<String>,
    pub reminders: Option<String>,
    pub date: String,
    pub color: i32,
    pub hide_content: bool,
    pub is_deleted: bool,
    pub is_archived: bool,
    pub is_list: bool,
    pub is_starred: bool,
    pub pin: Option<String>,
    pub password: Option<String>,
}

#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
pub struct SavingNote {
    pub note_id: i32,
    pub title: String,
    pub content: String,
    pub image_url: Option<String>,
    pub list_parse_string: Option<String>,
    pub reminders: Option<String>,
    pub date: String,
    pub color: i32,
    pub hide_content: bool,
    pub is_deleted: bool,
    pub is_archived: bool,
    pub is_list: bool,
    pub is_starred: bool,
    pub pin: Option<String>,
    pub password: Option<String>,
}

impl SavingNote {
    pub fn to_note(&self, account_id: i32) -> Note {
        Note {
            note_id: self.note_id,
            account_id,
            title: self.title.clone(),
            content: self.content.clone(),
            image_url: self.image_url.clone(),
            list_parse_string: self.list_parse_string.clone(),
            reminders: self.reminders.clone(),
            date: self.date.clone(),
            color: self.color,
            hide_content: self.hide_content,
            is_deleted: self.is_deleted,
            is_archived: self.is_archived,
            is_list: self.is_list,
            is_starred: self.is_starred,
            pin: self.pin.clone(),
            password: self.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NoteId {
    pub(crate) note_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Notes {
    pub(crate) notes: Vec<Note>,
}


#[cfg(test)]
pub mod tests {
    use crate::db;

    use super::*;

    /*
            #[test]
            pub fn check_create_note(){
                let connection = db::connect().get().unwrap();
                let note = Note{
                    note_id: 1,
                    account_id: 1,
                    title: "Test2".to_string(),
                    content: "Test2".to_string(),
                    image_url: None,
                    list_parse_string: None,
                    reminders: "".to_string(),
                    date: None,
                    color: None,
                    hide_content: None,
                    is_deleted: None,
                    is_archived: None,
                    is_list: None,
                    is_starred: None,
                    pin: None,
                    password: None
                };
                let response = Note::create_or_update(note, &connection);
                println!("{}", response.to_string());
            }*/
}