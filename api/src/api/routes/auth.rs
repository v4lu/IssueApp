use crate::api::{handlers::auth::*, middlewares::authentication_guard::AuthenticationGuard};
use actix_web::web;
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/refresh", web::post().to(refresh_token))
            .route("/oauth/github", web::get().to(init_github))
            .route("/oauth/github/callback", web::get().to(github_callback))
            .route("/oauth/github/login", web::get().to(login_with_github))
            .service(
                web::scope("")
                    .wrap(AuthenticationGuard)
                    .route("", web::get().to(session)),
            ),
    );
}
