use std::sync::Arc;

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, Scope, TokenUrl};
use sqlx::PgPool;

use crate::{
    config::Config,
    errors::CustomError,
    models::{
        auth::{CreateGithubUser, UpdateGithubUser, UserGithubResponse},
        github::{AccessTokenResponse, UserEmail, UserInfo},
    },
    repositories::user::UserRepository,
};

use super::token::TokenService;

pub struct OauthService {
    oauth_client: BasicClient,
    client_secret: String,
    client_id: String,
    user_repo: UserRepository,
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
            user_repo: UserRepository::new(pool),
            token_service,
        })
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

        let user_exist = self
            .user_repo
            .get_user_by_github_id(user_info.id.to_string())
            .await;

        if user_exist.is_err() {
            let create_user_data = CreateGithubUser {
                email: user_email.clone(),
                username: user_info.login.clone(),
                github_id: user_info.id.to_string(),
                github_url: user_info.html_url.clone(),
                avatar_url: Some(user_info.avatar_url.clone()),
            };

            self.user_repo
                .insert_github_user(create_user_data)
                .await
                .map_err(|e| CustomError::DatabaseError(e.to_string()))?;
        }

        let user = self
            .user_repo
            .get_user_by_github_id(user_info.id.to_string())
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        let email = user.email.clone();
        let mut update_user = UpdateGithubUser {
            email: Some(email.clone()),
            avatar_url: user.avatar_url.clone(),
            github_id: user.github_id.clone(),
            github_url: user.github_url.clone(),
        };
        let mut update_fields_count = 0;

        let user_info_html_url = user_info.html_url.clone();
        if user.github_url != Some(user_info_html_url.clone()) {
            update_user.github_url = Some(user_info_html_url);
            update_fields_count += 1;
        }

        let user_info_avatar_url = user_info.avatar_url.clone();
        if user.avatar_url != Some(user_info_avatar_url.clone()) {
            update_user.avatar_url = Some(user_info_avatar_url);
            update_fields_count += 1;
        }

        if email.clone() != user_email {
            update_user.email = Some(user_email);
            update_fields_count += 1;
        }

        let updated_user = if update_fields_count > 0 {
            Some(
                self.user_repo
                    .update_github_user(user.id, update_user)
                    .await
                    .map_err(|e| CustomError::DatabaseError(e.to_string()))?,
            )
        } else {
            None
        };

        let user = updated_user.unwrap_or(user);

        let (access_token, refresh_token, access_token_expiration, refresh_token_expiration) = self
            .token_service
            .generate_token_pair(user.id)
            .await
            .map_err(|_| CustomError::InternalServerError)?;

        let user_github_res = UserGithubResponse {
            user,
            access_token,
            refresh_token,
            access_token_expiration,
            refresh_token_expiration,
        };

        Ok(user_github_res)
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
