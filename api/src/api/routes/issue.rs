use actix_web::web;

use crate::api::{
    handlers::issue::*,
    middlewares::{authentication_guard::AuthenticationGuard, org_guard::OrgGuard},
};

pub fn configure_issue_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/issues/{org_id}")
            .wrap(AuthenticationGuard)
            .wrap(OrgGuard)
            .route("", web::post().to(create_issue))
            .route("", web::get().to(get_issues))
            .route("/{issue_id}", web::get().to(get_issue))
            .route("/{issue_id}", web::patch().to(update_issue))
            .route("/{issue_id}", web::delete().to(delete_issue)),
    );
}
