use crate::api::{
    handlers::org::*,
    middlewares::{
        authentication_guard::AuthenticationGuard, org_guard::OrgGuard, role_guard::RoleGuard,
    },
};
use crate::models::org::MemberRole;
use actix_web::web;

pub fn configure_organization_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/org")
            .wrap(AuthenticationGuard)
            .service(
                web::scope("/{org_id}")
                    .wrap(OrgGuard)
                    .route("", web::get().to(get_org))
                    .route("/session-role", web::get().to(get_session_role))
                    .service(
                        web::scope("/member")
                            .wrap(RoleGuard::new(vec![MemberRole::Owner]))
                            .route("/{member_id}", web::patch().to(update_member_role))
                            .route("/{member_id}", web::delete().to(remove_member)),
                    )
                    .service(
                        web::scope("")
                            .wrap(RoleGuard::new(vec![MemberRole::Admin, MemberRole::Owner]))
                            .route("", web::patch().to(update_org))
                            .route("", web::delete().to(delete_org))
                            .route("/invites", web::post().to(invite_user))
                            .route("/invites", web::get().to(list_org_invites))
                            .route("/members", web::get().to(list_org_members)),
                    ),
            )
            .route("", web::post().to(create_org))
            .route("", web::get().to(list_organizations)),
    );

    cfg.service(
        web::scope("/invites")
            .wrap(AuthenticationGuard)
            .route("", web::get().to(list_user_invites))
            .route("/{token}/{org_id}/accept", web::post().to(accept_invite))
            .route("/{token}", web::delete().to(cancel_invite)),
    );
}
