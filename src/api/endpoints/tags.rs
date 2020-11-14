use crate::responders::ApiResponse;
use rocket::http::Status;
use rocket_contrib::json::Json;
use crate::schemas::tags::{SavingTag, PatchingTag};
use crate::auth::claims::Token;
use crate::db::guard::Connection;
use crate::crud::tags::{tag_exists, tag_insert_if_empty, tag_patch_if_exists, tag_delete, tags_get_all, tags_get_existing};
use chrono::{Utc, TimeZone};
use crate::models::tags::Tag;
use rocket::Rocket;

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/tag", routes![
       add_tag,
       patch_tag,
       delete_tag,
       get_updated_tags,
       get_deleted_tags
    ])
}

#[post("/", data = "<json_tag>")]
fn add_tag(json_tag: Json<SavingTag>, token: Token, connection: Connection) -> ApiResponse {
    let saving_tag = json_tag.0;
    if tag_exists(&saving_tag.id, &token.sub, &connection) {
        return TAG_ALREADY_EXISTS;
    }
    let tag = saving_tag.to_tag(&token.sub);
    return match tag_insert_if_empty(tag, &connection) {
        Err(error) => {
            println!("Unable to insert note: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => TAG_ADD_SUCCESS
    };
}

#[patch("/<tag_id>", data = "<json_tag>")]
fn patch_tag(tag_id: String, json_tag: Json<PatchingTag>, token: Token, connection: Connection) -> ApiResponse {
    let patching_tag = json_tag.0;
    if patching_tag.last_modify_date.is_none() {
        return TAG_MISSING_LAST_MODIFY;
    }
    if !tag_exists(&tag_id, &token.sub, &connection) {
        return TAG_DOESNT_EXIST;
    }
    return match tag_patch_if_exists(tag_id, token.sub, patching_tag, &connection) {
        Err(error) => {
            println!("Unable to patch tag: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => TAG_PATCH_SUCCESS
    };
}

#[delete("/<tag_id>")]
fn delete_tag(tag_id: String, token: Token, connection: Connection) -> ApiResponse {
    if !tag_exists(&tag_id, &token.sub, &connection) {
        return TAG_DOESNT_EXIST;
    }
    return match tag_delete(tag_id, token.sub, &connection) {
        Err(error) => {
            println!("Unable to delete note: {}", error);
            INTERNAL_DB_ERROR
        }
        Ok(_changed) => TAG_DELETE_SUCCESS
    };
}

#[get("/list?<last_updated>")]
fn get_updated_tags(last_updated: u64, token: Token, connection: Connection) -> Result<String, ApiResponse> {
    return match tags_get_all(&token.sub, &connection) {
        Err(error) => {
            println!("Unable to get all tags: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(synced_notes) => {
            let last_updated = Utc.timestamp(last_updated as i64 / 1000, 0);
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

/// Route for getting a list of deleted notes based on a provided list of id's
#[post("/deleted", data = "<id_list>")]
fn get_deleted_tags(id_list: Json<Vec<String>>, token: Token, connection: Connection) -> Result<String, ApiResponse> {
    return match tags_get_existing(id_list.clone(), &token.sub, &connection) {
        Err(error) => {
            println!("Unable to get deleted tag id's: {}", error);
            Err(INTERNAL_DB_ERROR)
        }
        Ok(existing_list) => {
            // Since the repository method can only find what note_id's exist,
            // we need to convert it into a list of non-existing id's
            let deleted_list: Vec<String> = id_list.0.into_iter()
                .filter(|id| !existing_list.contains(&id)).collect();
            Ok(serde_json::to_string(&deleted_list).unwrap())
        }
    };
}

pub(crate) const TAG_ALREADY_EXISTS: ApiResponse = ApiResponse {
    body: "TagAlreadyExists",
    status: Status::BadRequest
};

pub(crate) const TAG_MISSING_LAST_MODIFY: ApiResponse = ApiResponse {
    body: "TagMissingLastModifyDate",
    status: Status::BadRequest
};

pub(crate) const TAG_DOESNT_EXIST: ApiResponse = ApiResponse {
    body: "TagDoesntExist",
    status: Status::BadRequest
};

pub(crate) const TAG_ADD_SUCCESS: ApiResponse = ApiResponse {
    body: "TagAddSuccess",
    status: Status::Ok
};

pub(crate) const TAG_PATCH_SUCCESS: ApiResponse = ApiResponse {
    body: "TagPatchSuccess",
    status: Status::Ok
};

pub(crate) const TAG_DELETE_SUCCESS: ApiResponse = ApiResponse {
    body: "TagDeleteSuccess",
    status: Status::Ok
};
pub(crate) const INTERNAL_DB_ERROR: ApiResponse = ApiResponse {
    body: "InternalDatabaseError",
    status: Status::InternalServerError
};