use diesel::expression::exists::exists;
use diesel::prelude::*;
use diesel::select;
#[cfg(test)]
use mocktopus::macros::*;

use crate::note::model::{Note, PatchingNote};
use crate::schema::notes;

#[cfg_attr(test, mockable)]
pub fn note_exists(account_id: &String, note_id: &String, connection: &PgConnection) -> bool {
    let note_exists = notes::dsl::notes.select(notes::note_id)
        .filter(notes::note_id.eq(note_id))
        .filter(notes::account_id.eq(account_id))
        .first::<String>(connection);
    return if note_exists.is_ok() {
        true
    } else {
        false
    };
}

#[cfg_attr(test, mockable)]
pub fn note_insert_if_empty(note: Note, connection: &PgConnection) -> Result<usize, String> {
    let insert_result = diesel::insert_into(notes::table)
        .values(&note)
        .on_conflict((notes::columns::note_id, notes::columns::account_id))
        .do_nothing()
        .execute(connection);
    return if insert_result.is_err() {
        Err(insert_result.err().unwrap().to_string())
    } else {
        Ok(insert_result.unwrap())
    };
}

#[cfg_attr(test, mockable)]
pub fn note_update_if_exists(note: Note, connection: &PgConnection) -> Result<usize, String> {
    let update_result = diesel::insert_into(notes::table)
        .values(&note)
        .on_conflict((notes::columns::note_id, notes::columns::account_id))
        .do_update()
        .set(&note)
        .execute(connection);
    return if update_result.is_err() {
        Err(update_result.err().unwrap().to_string())
    } else {
        Ok(update_result.unwrap())
    };
}

#[cfg_attr(test, mockable)]
pub fn note_patch_if_exists(account_id: String, note_id: String, note: PatchingNote, connection: &PgConnection) -> Result<usize, String> {
    let target = notes::dsl::notes.filter(
        notes::account_id.eq(&account_id))
        .filter(notes::note_id.eq(&note_id));
    return match diesel::update(target)
        .set(&note)
        .execute(connection) {
        Err(error) => Err(error.to_string()),
        Ok(usize) => Ok(usize)
    }
}


