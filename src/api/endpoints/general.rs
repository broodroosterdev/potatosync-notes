use rocket::Rocket;
use crate::auth::claims::Token;
use crate::db::guard::Connection;
use crate::crud::*;
use crate::models::notes::Note;
use crate::responders::ApiResponse;
use crate::api::endpoints::notes::INTERNAL_DB_ERROR;

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![last_changed])
}

#[get("/last_changed")]
fn last_changed(token: Token, connection: Connection) -> Result<String, ApiResponse> {
    let mut latest_note_change: i64 = 0;
    match notes::notes_get_all(&*token.sub, &connection.0){
        Ok(notes) => {
            latest_note_change = notes.iter()
                .map(|note| note.last_modify_date.timestamp_millis())
                .max()
                .unwrap_or(0);
        },
        Err(error) => {
            println!("Unable to get all notes: {}", error);
            return Err(INTERNAL_DB_ERROR);
        }
    }

    let mut latest_tag_change: i64 = 0;
    match tags::tags_get_all(&*token.sub, &connection.0){
        Ok(tags) => {
            latest_tag_change = tags.iter()
                .map(|tag| tag.last_modify_date.timestamp_millis())
                .max()
                .unwrap_or(0);
        },
        Err(error) => {
            println!("Unable to get all tags: {}", error);
            return Err(INTERNAL_DB_ERROR);
        }
    }

    let mut latest_setting_change: i64 = 0;
    match settings::get_all_settings(&*token.sub, &connection.0){
        Ok(settings) => {
            latest_setting_change = settings.iter()
                .map(|setting| setting.last_modify_date.timestamp_millis())
                .max()
                .unwrap_or(0);
        },
        Err(error) => {
            println!("Unable to get all settings: {}", error);
            return Err(INTERNAL_DB_ERROR);
        }
    }

    let timestamps = [latest_note_change, latest_tag_change, latest_setting_change];
    return Ok(timestamps.iter().max().unwrap().to_string());
}
