use crate::api::handlers::github::*;
use actix_web::web;

pub fn configure_github_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/github")
            .route("/repos", web::get().to(get_repos))
            .route("/repos/{owner}/{repo}", web::get().to(get_repository))
            .route(
                "/repos/{owner}/{repo}/issues",
                web::get().to(get_repository_issues),
            )
            .route("/repos/{owner}/{repo}/issues", web::post().to(create_issue))
            .route(
                "/repos/{owner}/{repo}/issues/{issue_number}",
                web::get().to(get_issue),
            )
            .route(
                "/repos/{owner}/{repo}/issues/{issue_number}",
                web::patch().to(update_issue),
            ),
    );
}
