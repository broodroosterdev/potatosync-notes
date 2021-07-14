mod last_changed;
mod ping;

use actix_web::web;


// Configure files app services
pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg
        //.service(last_changed::last_changed)// POST /last_changed
        .service(ping::ping)// GET /ping
        .service(ping::secure_ping);// GET /secure-ping
}