use actix_web::web;

use crate::handlers::general::*;

pub fn general_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/general")
            .route("/health_check", web::get().to(general_health_check)),
    );
}

