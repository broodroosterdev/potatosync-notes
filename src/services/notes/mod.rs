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
        .service(add::add)// POST /note
        .service(update::update) // PATCH /note/{id}
        .service(delete::delete) // DELETE /note/{id}
        .service(delete_all::delete_all)
        .service(get::get)                      // GET /note
        .service(deleted::deleted);             // POST /deleted
}