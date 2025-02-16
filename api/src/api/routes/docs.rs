use actix_web::web;

use crate::api::{
    handlers::docs::*,
    middlewares::{authentication_guard::AuthenticationGuard, org_guard::OrgGuard},
};

pub fn configure_docs_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/docs/{org_id}")
            .wrap(AuthenticationGuard)
            .wrap(OrgGuard)
            .route("", web::post().to(create_doc))
            .route("", web::get().to(get_all_org_docs))
            .route("/{doc_id}", web::get().to(get_doc))
            .route("/{doc_id}", web::patch().to(update_doc))
            .route("/{doc_id}", web::delete().to(delete_doc)),
    );
}
