use crate::schema::blobs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Insertable, Queryable, AsChangeset, Serialize)]
pub struct Blob {
    pub id: String,
    #[serde(skip_serializing)]
    pub account_id: String,
    pub content: String,
    pub last_changed: i64,
}
