use actix_web::web;

use crate::api::{
    handlers::comment::*,
    middlewares::{authentication_guard::AuthenticationGuard, org_guard::OrgGuard},
};

pub fn configure_comment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comments/{org_id}")
            .wrap(AuthenticationGuard)
            .wrap(OrgGuard)
            .route("", web::post().to(create_comment))
            .route("/all/{comment_owner_id}", web::get().to(get_comments))
            .route("/{comment_id}", web::get().to(get_comment))
            .route("/{comment_id}", web::patch().to(update_comment))
            .route("/{comment_id}", web::delete().to(delete_comment)),
    );
}
