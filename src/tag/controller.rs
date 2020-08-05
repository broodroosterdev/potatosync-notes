use crate::tag::repository::{tag_exists, tag_insert_if_empty, tag_patch_if_exists, tags_get_all, tags_get_existing, tag_delete};
use crate::tag::model::{SavingTag, PatchingTag, Tag};
use diesel::PgConnection;
use crate::responders::ApiResponse;
use crate::tag::responses::{TAG_ALREADY_EXISTS, TAG_ADD_SUCCESS, TAG_MISSING_LAST_MODIFY, TAG_DOESNT_EXIST, TAG_PATCH_SUCCESS, TAG_DELETE_SUCCESS};
use crate::responses::INTERNAL_DB_ERROR;
use chrono::{Utc, TimeZone};

///Adds tag and if it already exist, it will return an error
pub(crate) fn add_tag(saving_tag: SavingTag, account_id: String, connection: &PgConnection) -> ApiResponse {
    if tag_exists(&saving_tag.id, &account_id, connection) {
        return TAG_ALREADY_EXISTS;
    }
    let tag = saving_tag.to_tag(&account_id);
    return match tag_insert_if_empty(tag, connection) {
        Err(error) => {
            println!("Unable to insert note: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => TAG_ADD_SUCCESS
    };
}

/// Only change tag fields that have changed, if no last_modify_date is provided, it will return an error.
pub(crate) fn update_tag(patching_tag: PatchingTag, tag_id: String, account_id: String, connection: &PgConnection) -> ApiResponse {
    if patching_tag.last_modify_date.is_none() {
        return TAG_MISSING_LAST_MODIFY;
    }
    if !tag_exists(&tag_id, &account_id, connection) {
        return TAG_DOESNT_EXIST;
    }
    return match tag_patch_if_exists(tag_id, account_id, patching_tag, connection) {
        Err(error) => {
            println!("Unable to patch tag: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => TAG_PATCH_SUCCESS
    };
}

pub(crate) fn delete_tag(tag_id: String, account_id: String, connection: &PgConnection) -> ApiResponse {
    if !tag_exists(&tag_id, &account_id, connection) {
        return TAG_DOESNT_EXIST;
    }
    return match tag_delete(tag_id, account_id, connection) {
        Err(error) => {
            println!("Unable to delete note: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => TAG_DELETE_SUCCESS
    };
}

/// Get list of tags updated after provided timestamp
pub(crate) fn get_updated_tags(last_updated: i64, account_id: String, connection: &PgConnection)
                               -> Result<String, ApiResponse> {
    return match tags_get_all(&account_id, connection) {
        Err(error) => {
            println!("Unable to get all tags: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(synced_notes) => {
            let last_updated = Utc.timestamp(last_updated / 1000, 0);
            let mut updated_tags: Vec<Tag> = vec![];
            for tag in synced_notes {
                if !tag.last_modify_date.le(&last_updated) {
                    updated_tags.push(tag);
                }
            }
            Ok(serde_json::to_string(&updated_tags).unwrap())
        }
    };
}

pub(crate) fn get_deleted_tags(id_list: Vec<String>, account_id: String, connection: &PgConnection) -> Result<String, ApiResponse> {
    return match tags_get_existing(id_list.clone(), &account_id, connection) {
        Err(error) => {
            println!("Unable to get deleted tag id's: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(existing_list) => {
            // Since the repository method can only find what note_id's exist,
            // we need to convert it into a list of non-existing id's
            let deleted_list: Vec<String> = id_list.into_iter()
                .filter(|id| !existing_list.contains(&id)).collect();
            Ok(serde_json::to_string(&deleted_list).unwrap())
        }
    };
}
