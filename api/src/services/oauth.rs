use std::sync::Arc;

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, Scope, TokenUrl};
use sqlx::PgPool;

use crate::{
    config::Config,
    errors::CustomError,
    models::{
        auth::{CreateGithubUser, UpdateGithubUser, User, UserGithubResponse},
        github::{AccessTokenResponse, UserEmail, UserInfo},
        user_preferences::UserPreferenceRequest,
    },
    repositories::{user::UserRepository, user_preferences::UserPreferencesRepository},
};

use super::token::TokenService;

pub struct OauthService {
    oauth_client: BasicClient,
    client_secret: String,
    client_id: String,
    user_repo: UserRepository,
    user_preferences_repo: UserPreferencesRepository,
    token_service: Arc<TokenService>,
}

impl OauthService {
    pub fn new(
        pool: PgPool,
        config: &Config,
        token_service: Arc<TokenService>,
    ) -> Result<Self, CustomError> {
        let oauth_client = BasicClient::new(
            ClientId::new(config.github_client_id.clone()),
            Some(ClientSecret::new(config.github_client_secret.clone())),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?,
            Some(
                TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                    .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?,
            ),
        );

        Ok(Self {
            oauth_client,
            client_id: config.github_client_id.clone(),
            client_secret: config.github_client_secret.clone(),
            user_preferences_repo: UserPreferencesRepository::new(pool.clone()),
            user_repo: UserRepository::new(pool),
            token_service,
        })
    }

    pub async fn init_github_link(&self) -> reqwest::Url {
        let (authorize_url, _) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .add_scope(Scope::new("read:user".to_string()))
            .url();

        authorize_url
    }

    pub async fn init_github_oauth(&self) -> reqwest::Url {
        let (authorize_url, _) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .add_scope(Scope::new("read:user".to_string()))
            .add_scope(Scope::new("repo".to_string()))
            .add_scope(Scope::new("read:org".to_string()))
            .url();

        authorize_url
    }

    pub async fn get_github_access_token(
        &self,
        code: String,
    ) -> Result<AccessTokenResponse, CustomError> {
        self.exchange_code_for_token(code).await
    }

    pub async fn handle_github_callback(
        &self,
        code: String,
    ) -> Result<UserGithubResponse, CustomError> {
        let token_res = self.exchange_code_for_token(code).await?;
        let (user_info, user_email) = self.get_user_info(&token_res.access_token).await?;

        let mut user = self.get_or_create_user(&user_info, &user_email).await?;

        if let Some(updated_user) = self
            .update_user_if_needed(&user, &user_info, &user_email)
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
            access_token_expiration,
            refresh_token_expiration,
        })
    }

    async fn get_or_create_user(
        &self,
        user_info: &UserInfo,
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
        user_info: &UserInfo,
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
        user_info: &UserInfo,
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

    async fn exchange_code_for_token(
        &self,
        code: String,
    ) -> Result<AccessTokenResponse, CustomError> {
        let token_url = self
            .oauth_client
            .token_url()
            .ok_or(CustomError::InternalServerError)?;

        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
        ];

        let client = reqwest::Client::new();
        let res = client
            .post(token_url.as_str())
            .header("Accept", "application/json")
            .header("User-Agent", "rust-github-oauth")
            .form(&params) // Use form() instead of json()
            .send()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        if !res.status().is_success() {
            let error_body = res
                .text()
                .await
                .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;
            log::error!("GitHub API error: {}", error_body);
            return Err(CustomError::ExternalServiceError(format!(
                "GitHub API error: {}",
                error_body
            )));
        }

        let token_res: AccessTokenResponse = res
            .json()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        Ok(token_res)
    }

    async fn get_user_emails(&self, access_token: &str) -> Result<Vec<UserEmail>, CustomError> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.github.com/user/emails")
            .header("User-Agent", "rust-github-oauth")
            .header("Accept", "application/json")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| {
                log::error!("Request error: {:?}", e);
                CustomError::ExternalServiceError(e.to_string())
            })?;

        if !res.status().is_success() {
            let error_body = res
                .text()
                .await
                .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;
            log::error!("GitHub API error response: {}", error_body);
            return Err(CustomError::ExternalServiceError(format!(
                "GitHub API error: {}",
                error_body
            )));
        }

        let emails: Vec<UserEmail> = res.json().await.map_err(|e| {
            log::error!("JSON parsing error: {:?}", e);
            CustomError::ExternalServiceError(e.to_string())
        })?;

        Ok(emails)
    }

    async fn get_user_info(&self, access_token: &str) -> Result<(UserInfo, String), CustomError> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.github.com/user")
            .header("User-Agent", "rust-github-oauth")
            .header("Accept", "application/json")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| {
                log::error!("Request error: {:?}", e);
                CustomError::ExternalServiceError(e.to_string())
            })?;

        // Log the status code
        log::info!("GitHub API response status: {}", res.status());

        if !res.status().is_success() {
            let error_body = res
                .text()
                .await
                .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;
            log::error!("GitHub API error response: {}", error_body);
            return Err(CustomError::ExternalServiceError(format!(
                "GitHub API error: {}",
                error_body
            )));
        }

        let body = res.text().await.map_err(|e| {
            log::error!("Error reading response body: {:?}", e);
            CustomError::ExternalServiceError(e.to_string())
        })?;
        log::info!("GitHub API response body: {}", body);

        let user_info = serde_json::from_str::<UserInfo>(&body).map_err(|e| {
            log::error!("JSON parsing error: {:?}", e);
            CustomError::ExternalServiceError(e.to_string())
        });

        let emails = self.get_user_emails(access_token).await?;

        let primary_email = emails.into_iter().find(|e| e.primary).map(|e| e.email);

        let mut user_info = user_info?;
        user_info.email = primary_email;

        let email = user_info
            .email
            .clone()
            .ok_or(CustomError::ExternalServiceError(
                "Primary email not found".to_string(),
            ))?;

        Ok((user_info, email))
    }
}
