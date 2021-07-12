use crate::models::{Note, Tag};
use crate::errors::ApiError;
use config::Source;
use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods, PgArrayExpressionMethods};
use crate::services::notes::update::NoteTemplate;
use crate::services::tags::update::TagTemplate;
use diesel::dsl::max;


pub fn add_note(conn: &PgConnection, note: &Note) -> Result<usize, ApiError> {
    use crate::schema::notes::dsl::*;
    diesel::insert_into(notes)
        .values(note)
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn update_note(conn: &PgConnection, note_id: &String, data: &NoteTemplate, given_account_id: &str) -> Result<usize, ApiError> {
    use crate::schema::notes::dsl::*;
    diesel::update(notes.filter(id.eq(note_id).and(account_id.eq(given_account_id))))
        .set((
                content.eq(&data.content),
                last_changed.eq(data.last_changed as i64)
            ))
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn delete_note(conn: &PgConnection, note_id: &String, given_account_id: &str) -> Result<usize, ApiError> {
    use crate::schema::notes::dsl::*;
    diesel::delete(notes)
        .filter(id.eq(note_id).and(account_id.eq(given_account_id)))
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn delete_all_note(conn: &PgConnection, given_account_id: &str) -> Result<usize, ApiError> {
    use crate::schema::notes::dsl::*;
    diesel::delete(notes)
        .filter(account_id.eq(given_account_id))
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_notes(conn: &PgConnection, given_account_id: &str) -> Result<Vec<Note>, ApiError> {
    use crate::schema::notes::dsl::*;
    notes.filter(account_id.eq(given_account_id))
        .load::<Note>(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_notes_updated_after(conn: &PgConnection, updated_after: u64, given_account_id: &str) -> Result<Vec<Note>, ApiError> {
    use crate::schema::notes::dsl::*;
    notes.filter(account_id.eq(given_account_id).and(last_changed.gt(updated_after as i64)))
        .load::<Note>(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn add_tag(conn: &PgConnection, tag: &Tag) -> Result<usize, ApiError> {
    use crate::schema::tags::dsl::*;
    diesel::insert_into(tags)
        .values(tag)
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn update_tag(conn: &PgConnection, tag_id: &String, data: &TagTemplate, given_account_id: &str) -> Result<usize, ApiError> {
    use crate::schema::tags::dsl::*;
    diesel::update(tags.filter(id.eq(tag_id).and(account_id.eq(given_account_id))))
        .set((
            content.eq(&data.content),
            last_changed.eq(data.last_changed as i64)
            ))
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn delete_tag(conn: &PgConnection, note_id: &String, given_account_id: &str) -> Result<usize, ApiError> {
    use crate::schema::tags::dsl::*;
    diesel::delete(tags)
        .filter(id.eq(note_id).and(account_id.eq(given_account_id)))
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn delete_all_tag(conn: &PgConnection, given_account_id: &str) -> Result<usize, ApiError> {
    use crate::schema::tags::dsl::*;
    diesel::delete(tags)
        .filter(account_id.eq(given_account_id))
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_tags(conn: &PgConnection, given_account_id: &str) -> Result<Vec<Tag>, ApiError> {
    use crate::schema::tags::dsl::*;
    tags.filter(account_id.eq(given_account_id))
        .load::<Tag>(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_tags_updated_after(conn: &PgConnection, updated_after: u64, given_account_id: &str) -> Result<Vec<Tag>, ApiError> {
    use crate::schema::tags::dsl::*;
    tags.filter(account_id.eq(given_account_id).and(last_changed.gt(updated_after as i64)))
        .load::<Tag>(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_last_note_changed(conn: &PgConnection, given_account_id: &str) -> Result<Option<i64>, ApiError> {
    use crate::schema::*;
    notes::table
        .filter(notes::account_id.eq(given_account_id))
        .select(max(notes::last_changed))
        .first(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_last_tag_changed(conn: &PgConnection, given_account_id: &str) -> Result<Option<i64>, ApiError> {
    use crate::schema::*;
    tags::table
        .filter(tags::account_id.eq(given_account_id))
        .select(max(tags::last_changed))
        .first(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_existing_notes(conn: &PgConnection, id_list: &Vec<String>, given_account_id: &str) -> Result<Vec<String>, ApiError> {
    use crate::schema::notes;
    notes::table
        .filter(notes::account_id.eq(given_account_id).and(notes::id.eq_any(id_list.clone())))
        .select(notes::id)
        .load::<String>(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_existing_tags(conn: &PgConnection, id_list: &Vec<String>, given_account_id: &str) -> Result<Vec<String>, ApiError> {
    use crate::schema::tags;
    tags::table
        .filter(tags::account_id.eq(given_account_id).and(tags::id.eq_any(id_list.clone())))
        .select(tags::id)
        .load::<String>(conn)
        .map_err(|error| ApiError::DBError(error))
}