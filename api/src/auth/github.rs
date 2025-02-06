use std::collections::HashMap;

use reqwest::{header, Client};

use crate::{
    errors::CustomError,
    models::github::{GithubEmail, GithubUser},
};

pub struct GithubAuth {
    client_id: String,
    client_secret: String,
    redirect_url: String,
    http_client: Client,
}

impl GithubAuth {
    pub fn new(client_id: String, client_secret: String, redirect_url: String) -> Self {
        let http_client = Client::new();
        Self {
            client_id,
            client_secret,
            redirect_url,
            http_client,
        }
    }

    pub fn get_authorize_url(&self) -> String {
        let mut url = reqwest::Url::parse("https://github.com/login/oauth/authorize").unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_url)
            .append_pair("scope", "user:email");

        url.to_string()
    }

    pub async fn exchange_code_for_token(&self, code: String) -> Result<String, CustomError> {
        let mut params = HashMap::new();
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);
        params.insert("code", &code);
        params.insert("redirect_uri", &self.redirect_url);

        let response = self
            .http_client
            .post("https://github.com/login/oauth/access_token")
            .header(header::ACCEPT, "application/json")
            .json(&params)
            .send()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        #[derive(serde::Deserialize)]
        struct TokenResponse {
            access_token: String,
        }

        let token_data = response
            .json::<TokenResponse>()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        Ok(token_data.access_token)
    }

    pub async fn get_user_data(&self, access_token: &str) -> Result<GithubUser, CustomError> {
        let response = self
            .http_client
            .get("https://api.github.com/user")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .header(header::USER_AGENT, "rust-app")
            .send()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        let user_data = response
            .json::<GithubUser>()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        Ok(user_data)
    }

    pub async fn get_user_emails(
        &self,
        access_token: &str,
    ) -> Result<Vec<GithubEmail>, CustomError> {
        let response = self
            .http_client
            .get("https://api.github.com/user/emails")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .header(header::USER_AGENT, "rust-app")
            .send()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;

        let emails = response
            .json::<Vec<GithubEmail>>()
            .await
            .map_err(|e| CustomError::ExternalServiceError(e.to_string()))?;
        Ok(emails)
    }

    pub fn extract_primary_email(&self, emails: Vec<GithubEmail>) -> Option<String> {
        emails
            .iter()
            .find(|email| email.primary && email.verified)
            .map(|email| email.email.clone())
    }
}
