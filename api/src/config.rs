use std::env;

use crate::errors::CustomError;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_url: String,
}

impl Config {
    pub fn new() -> Result<Self, CustomError> {
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .map_err(|_| CustomError::ConfigError("PORT environment variable not set".to_string()))?
            .parse()
            .map_err(|_| CustomError::ConfigError("Failed to parse PORT as u16".to_string()))?;

        let database_url = env::var("DATABASE_URL").map_err(|_| {
            CustomError::ConfigError("DATABASE_URL environment variable not set".to_string())
        })?;

        let github_client_id = env::var("GITHUB_CLIENT_ID").map_err(|_| {
            CustomError::ConfigError("GITHUB_CLIENT_ID environment variable not set".to_string())
        })?;

        let github_client_secret = env::var("GITHUB_CLIENT_SECRET").map_err(|_| {
            CustomError::ConfigError(
                "GITHUB_CLIENT_SECRET environment variable not set".to_string(),
            )
        })?;

        let github_redirect_url = env::var("GITHUB_REDIRECT_URL").map_err(|_| {
            CustomError::ConfigError("GITHUB_REDIRECT_URL environment variable not set".to_string())
        })?;

        Ok(Config {
            port,
            database_url,
            github_client_id,
            github_client_secret,
            github_redirect_url,
        })
    }
}
