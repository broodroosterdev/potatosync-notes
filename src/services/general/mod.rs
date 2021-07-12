mod last_changed;

use actix_web::web;


// Configure files app services
pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(last_changed::last_changed);   // POST /last_changed
}