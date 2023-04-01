use actix_web::web;

use crate::handlers::users::*;

pub fn user_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/register", web::post().to(user_register))
            .route("/login", web::post().to(user_login))
            .route("/logout", web::post().to(user_logout))
            .route("/profile", web::get().to(user_profile))
            .route("/update", web::put().to(user_update))
            .route("/delete", web::delete().to(user_delete))
            .route("/verify", web::post().to(user_verify)),
    );
}
