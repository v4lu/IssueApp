use actix_web::web;

use crate::api::{
    handlers::user_preferences::*, middlewares::authentication_guard::AuthenticationGuard,
};

pub fn configure_user_preferences_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user-preferences")
            .wrap(AuthenticationGuard)
            .route("", web::post().to(create_user_preferences))
            .route("", web::get().to(get_user_preferences))
            .route("", web::patch().to(update_user_preference)),
    );
}
