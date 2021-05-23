use actix_web::web;

pub mod add;
pub mod update;
pub mod delete;
pub mod get;

// Configure files app services
pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(add::add)// POST /note
        .service(update::update) // PATCH /note/{id}
        .service(delete::delete) // DELETE /note/{id}
        .service(get::get);                      // GET /note
}