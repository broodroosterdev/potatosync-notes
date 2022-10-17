mod delete_all;
mod get;
mod upsert;

use actix_web::web;

// Configure files app services
pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_all::delete_all);
    cfg.service(get::get);
    cfg.service(upsert::upsert);
}

