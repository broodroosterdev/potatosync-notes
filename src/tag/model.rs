use chrono::{DateTime, Utc};
use crate::serde_helper::*;
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

/// Tag as provided by the client. Note the missing account_id since the client doesnt know it.
#[derive(Serialize, Deserialize)]
pub struct SavingTag {
    pub id: String,
    pub name: String,
    pub color: i32,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub last_modify_date: DateTime<Utc>
}

impl SavingTag {
    pub(crate) fn to_tag(&self, account_id: &String) -> Tag{
        Tag{
            id: self.id.clone(),
            account_id: account_id.clone(),
            name: self.name.clone(),
            color: self.color.clone(),
            last_modify_date: self.last_modify_date.clone()
        }
    }
}

#[table_name = "tags"]
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset, PartialEq, Clone)]
pub struct PatchingTag {
    pub name: Option<String>,
    pub color: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option")]
    #[serde(serialize_with = "serialize_option")]
    pub last_modify_date: Option<DateTime<Utc>>,
}
