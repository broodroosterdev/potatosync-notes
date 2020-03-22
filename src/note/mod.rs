use diesel::prelude::*;
use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};

use crate::db;
use crate::schema::notes;

use self::model::Note;

pub mod model;
pub mod controller;

#[cfg(test)]
mod tests {
    use model::tests;

    use super::*;

    /*
            #[test]
            fn check_getting_note(){
                tests::check_create_note();
                let connection = db::connect().get().unwrap();
                let result: Note = Note::get_by_id(1, &connection);
                println!("{}", result.title);
            }*/
}
