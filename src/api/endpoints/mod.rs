use rocket::Rocket;

pub mod notes;
pub mod settings;
pub mod tags;

pub fn fuel(rocket: Rocket) -> Rocket {
    let mut rocket = rocket;
    rocket = notes::fuel(rocket);
    rocket = settings::fuel(rocket);
    rocket = tags::fuel(rocket);
    rocket
}