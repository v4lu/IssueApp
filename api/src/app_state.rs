use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    config::Config,
    services::{
        auth::AuthService, comment::CommentService, issue::IssueService, oauth::OauthService,
        org::OrgService, token::TokenService, user_preferences::UserPreferencesService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub token_service: Arc<TokenService>,
    pub org_service: Arc<OrgService>,
    pub issue_service: Arc<IssueService>,
    pub user_preferences_service: Arc<UserPreferencesService>,
    pub comment_service: Arc<CommentService>,
    pub oauth_service: Arc<OauthService>,
}

impl AppState {
    pub async fn new(pool: PgPool, config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let token_service = Arc::new(TokenService::new().unwrap());

        let oauth_service = Arc::new(OauthService::new(
            pool.clone(),
            &config,
            token_service.clone(),
        )?);

        Ok(AppState {
            token_service: token_service.clone(),
            auth_service: Arc::new(AuthService::new(pool.clone(), token_service)),
            org_service: Arc::new(OrgService::new(pool.clone())),
            user_preferences_service: Arc::new(UserPreferencesService::new(pool.clone())),
            issue_service: Arc::new(IssueService::new(pool.clone())),
            comment_service: Arc::new(CommentService::new(pool.clone())),
            oauth_service,
        })
    }
}
