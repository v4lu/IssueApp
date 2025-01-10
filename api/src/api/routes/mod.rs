use actix_web::web;

mod auth;
mod comment;
mod github;
mod issue;
mod org;
mod user_preferences;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(auth::configure_auth_routes)
            .configure(org::configure_organization_routes)
            .configure(issue::configure_issue_routes)
            .configure(user_preferences::configure_user_preferences_routes)
            .configure(github::configure_github_routes)
            .configure(comment::configure_comment_routes),
    );
}
