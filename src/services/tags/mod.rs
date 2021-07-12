use actix_web::web;

pub mod add;
pub mod update;
pub mod delete;
pub mod get;
pub mod delete_all;
pub mod deleted;


// Configure files app services
pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(add::add)// POST /tag
        .service(update::update) // PATCH /tag/{id}
        .service(delete::delete) // DELETE /tag/{id}
        .service(delete_all::delete_all) // DELETE /tag/all
        .service(get::get)                      // GET /tag(?last_updated=0)
        .service(deleted::deleted); // POST /deleted
}