use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    auth::github::GithubAuth,
    config::Config,
    errors::CustomError,
    models::{
        auth::{CreateGithubUser, UpdateGithubUser, User, UserGithubResponse},
        github::GithubUser,
        user_preferences::UserPreferenceRequest,
    },
    repositories::{user::UserRepository, user_preferences::UserPreferencesRepository},
};

use super::token::TokenService;

pub struct OauthService {
    user_repo: UserRepository,
    user_preferences_repo: UserPreferencesRepository,
    token_service: Arc<TokenService>,
    github_service: Arc<GithubAuth>,
}

impl OauthService {
    pub fn new(
        pool: PgPool,
        config: &Config,
        token_service: Arc<TokenService>,
    ) -> Result<Self, CustomError> {
        let github_service = Arc::new(GithubAuth::new(
            config.github_client_id.clone(),
            config.github_client_secret.clone(),
            config.github_redirect_server.clone(),
        ));
        Ok(Self {
            user_preferences_repo: UserPreferencesRepository::new(pool.clone()),
            user_repo: UserRepository::new(pool),
            token_service,
            github_service,
        })
    }

    pub async fn init_github_link(&self) -> String {
        self.github_service.get_authorize_url()
    }

    pub async fn get_github_access_token(&self, code: String) -> Result<String, CustomError> {
        self.github_service.exchange_code_for_token(code).await
    }

    pub async fn handle_github_callback(
        &self,
        code: String,
    ) -> Result<UserGithubResponse, CustomError> {
        let token_res = self.github_service.exchange_code_for_token(code).await?;

        let user_info = self.github_service.get_user_data(&token_res).await?;
        let emails = self.github_service.get_user_emails(&token_res).await?;
        let primary_email = self
            .github_service
            .extract_primary_email(emails)
            .ok_or(CustomError::InternalServerError)?;
        let mut user = self.get_or_create_user(&user_info, &primary_email).await?;

        if let Some(updated_user) = self
            .update_user_if_needed(&user, &user_info, &primary_email)
            .await?
        {
            user = updated_user;
        }

        let (access_token, refresh_token, access_token_expiration, refresh_token_expiration) = self
            .token_service
            .generate_token_pair(user.id)
            .await
            .map_err(|_| CustomError::InternalServerError)?;

        Ok(UserGithubResponse {
            user,
            access_token,
            refresh_token,
            access_token_expiration: access_token_expiration * 60,
            refresh_token_expiration: refresh_token_expiration * 60,
        })
    }

    async fn get_or_create_user(
        &self,
        user_info: &GithubUser,
        user_email: &str,
    ) -> Result<User, CustomError> {
        match self
            .user_repo
            .get_user_by_github_id(user_info.id.to_string())
            .await
        {
            Ok(user) => Ok(user),
            Err(_) => {
                let create_user_data = CreateGithubUser {
                    email: user_email.to_string(),
                    username: user_info.login.clone(),
                    github_id: user_info.id.to_string(),
                    github_url: user_info.html_url.clone(),
                    avatar_url: Some(user_info.avatar_url.clone()),
                };

                let user = self
                    .user_repo
                    .insert_github_user(create_user_data)
                    .await
                    .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

                self.user_preferences_repo
                    .create_user_preference(UserPreferenceRequest { user_id: user.id })
                    .await
                    .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

                Ok(user)
            }
        }
    }

    async fn update_user_if_needed(
        &self,
        user: &User,
        user_info: &GithubUser,
        user_email: &str,
    ) -> Result<Option<User>, CustomError> {
        let mut update_user = UpdateGithubUser {
            email: None,
            avatar_url: None,
            github_id: user.github_id.clone(),
            github_url: None,
        };

        let needs_update = self.collect_user_updates(&mut update_user, user, user_info, user_email);

        if needs_update {
            self.user_repo
                .update_github_user(user.id, update_user)
                .await
                .map(Some)
                .map_err(|e| CustomError::DatabaseError(e.to_string()))
        } else {
            Ok(None)
        }
    }

    fn collect_user_updates(
        &self,
        update_user: &mut UpdateGithubUser,
        user: &User,
        user_info: &GithubUser,
        user_email: &str,
    ) -> bool {
        let mut needs_update = false;

        if user.github_url.as_ref() != Some(&user_info.html_url) {
            update_user.github_url = Some(user_info.html_url.clone());
            needs_update = true;
        }

        if user.avatar_url.as_ref() != Some(&user_info.avatar_url) {
            update_user.avatar_url = Some(user_info.avatar_url.clone());
            needs_update = true;
        }

        if user.email != user_email {
            update_user.email = Some(user_email.to_string());
            needs_update = true;
        }

        needs_update
    }
}
