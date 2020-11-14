use diesel::{PgConnection, RunQueryDsl, ExpressionMethods};
use crate::schema::tags;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::query_dsl::select_dsl::SelectDsl;
use crate::models::tags::Tag;
use crate::schemas::tags::PatchingTag;

#[cfg_attr(test, mockable)]
pub fn tag_insert_if_empty(tag: Tag, connection: &PgConnection) -> Result<usize, String>{
    let insert_result = diesel::insert_into(tags::table)
        .values(&tag)
        .on_conflict_do_nothing()
        .execute(connection);
    return match insert_result {
        Err(error) => Err(error.to_string()),
        Ok(rows_updated) => Ok(rows_updated)
    }
}

#[cfg_attr(test, mockable)]
pub fn tag_exists(tag_id: &String, account_id: &String, connection: &PgConnection) -> bool {
    let tag_exists = tags::dsl::tags.select(tags::id)
        .filter(tags::id.eq(tag_id))
        .filter(tags::account_id.eq(account_id))
        .first::<String>(connection);
    return tag_exists.is_ok();
}

#[cfg_attr(test, mockable)]
pub fn tag_patch_if_exists(tag_id: String, account_id: String, tag: PatchingTag, connection: &PgConnection) -> Result<usize, String> {
    let target = tags::dsl::tags.filter(tags::account_id.eq(&account_id))
        .filter(tags::id.eq(&tag_id));
    return match diesel::update(target)
        .set(&tag)
        .execute(connection) {
        Err(error) => Err(error.to_string()),
        Ok(usize) => Ok(usize)
    }
}

#[cfg_attr(test, mockable)]
pub fn tags_get_all(account_id: &str, connection: &PgConnection) -> Result<Vec<Tag>, String> {
    let get_result: Result<Vec<Tag>, diesel::result::Error> = tags::dsl::tags.filter(tags::account_id.eq(account_id)).load::<Tag>(connection);
    return match get_result {
        Err(error) => Err(error.to_string()),
        Ok(notes) => Ok(notes)
    }
}

#[cfg_attr(test, mockable)]
pub fn tags_get_existing( id_list: Vec<String>, account_id: &str, connection: &PgConnection) -> Result<Vec<String>, String> {
    let get_result = tags::dsl::tags.select(tags::id).filter(tags::account_id.eq(account_id)).filter(tags::id.eq_any(id_list)).load::<String>(connection);
    return match get_result{
        Err(error) => Err(error.to_string()),
        Ok(existing_list) => Ok(existing_list)
    }
}

#[cfg_attr(test, mockable)]
pub fn tag_delete(tag_id: String, account_id: String, connection: &PgConnection) -> Result<usize, String> {
    let delete_result = diesel::delete(tags::table)
        .filter(tags::account_id.eq(&account_id))
        .filter(tags::id.eq(&tag_id))
        .execute(connection);
    return match delete_result {
        Err(error) => Err(error.to_string()),
        Ok(rows_changed) => Ok(rows_changed)
    }
}

