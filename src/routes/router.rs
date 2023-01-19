use crate::controllers::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(index::ping));
}
