#[cfg(test)]
use mocktopus::macros::*;
use diesel::prelude::*;
use crate::schema::notes;
use crate::models::notes::Note;
use crate::schemas::notes::PatchingNote;

#[cfg_attr(test, mockable)]
pub fn note_exists(account_id: &String, note_id: &String, connection: &PgConnection) -> bool {
    let note_exists = notes::dsl::notes.select(notes::note_id)
        .filter(notes::note_id.eq(note_id))
        .filter(notes::account_id.eq(account_id))
        .first::<String>(connection);
    return note_exists.is_ok();
}

#[cfg_attr(test, mockable)]
pub fn note_insert_if_empty(note: Note, connection: &PgConnection) -> Result<usize, String> {
    let insert_result = diesel::insert_into(notes::table)
        .values(&note)
        .on_conflict_do_nothing()
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

#[cfg_attr(test, mockable)]
pub fn note_delete(account_id: String, note_id: String, connection: &PgConnection) -> Result<usize, String> {
    let delete_result = diesel::delete(notes::table)
        .filter(notes::note_id.eq(&note_id))
        .filter(notes::account_id.eq(&account_id))
        .execute(connection);
    return match delete_result {
        Err(error) => Err(error.to_string()),
        Ok(usize) => Ok(usize)
    }
}

#[cfg_attr(test, mockable)]
pub fn note_delete_all(account_id: String, connection: &PgConnection) -> Result<usize, String> {
    let delete_result = diesel::delete(notes::table)
        .filter(notes::account_id.eq(&account_id))
        .execute(connection);
    return match delete_result {
        Err(error) => Err(error.to_string()),
        Ok(usize) => Ok(usize)
    }
}

#[cfg_attr(test, mockable)]
pub fn notes_get_all(account_id: &str, connection: &PgConnection) -> Result<Vec<Note>, String> {
    let get_result: Result<Vec<Note>, diesel::result::Error> = notes::dsl::notes.filter(notes::account_id.eq(account_id)).load::<Note>(connection);
    return match get_result {
        Err(error) => Err(error.to_string()),
        Ok(notes) => Ok(notes)
    }
}

#[cfg_attr(test, mockable)]
pub fn notes_get_existing(account_id: &str, id_list: Vec<String>, connection: &PgConnection) -> Result<Vec<String>, String> {
    let get_result = notes::dsl::notes.select(notes::note_id).filter(notes::account_id.eq(account_id)).filter(notes::note_id.eq_any(id_list)).load::<String>(connection);
    return match get_result{
        Err(error) => Err(error.to_string()),
        Ok(existing_list) => Ok(existing_list)
    }
}
