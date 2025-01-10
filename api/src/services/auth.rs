use futures::TryFutureExt;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    errors::CustomError,
    models::{
        auth::{
            CreateUserData, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, User,
        },
        user_preferences::UserPreferenceRequest,
    },
    repositories::{user::UserRepository, user_preferences::UserPreferencesRepository},
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

use super::token::TokenService;

pub struct AuthService {
    user_repo: UserRepository,
    token_service: Arc<TokenService>,
    user_preferences_repo: UserPreferencesRepository,
}

impl AuthService {
    pub fn new(pool: PgPool, token_service: Arc<TokenService>) -> Self {
        Self {
            user_repo: UserRepository::new(pool.clone()),
            token_service,
            user_preferences_repo: UserPreferencesRepository::new(pool.clone()),
        }
    }

    pub async fn register_user(
        &self,
        request: RegisterRequest,
    ) -> Result<RegisterResponse, CustomError> {
        let user_exist = self
            .user_repo
            .user_exists(request.email.clone(), request.username.clone())
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        if user_exist {
            return Err(CustomError::Conflict(
                "User with this username or email already exists".to_string(),
                "username or email".to_string(),
            ));
        }

        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(request.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let create_user_data = CreateUserData {
            email: request.email,
            username: request.username,
            password: password_hash,
        };

        let user = self
            .user_repo
            .create_user(create_user_data)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        self.user_preferences_repo
            .create_user_preference(UserPreferenceRequest { user_id: user.id })
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        Ok(RegisterResponse {
            message: "User registered successfully".to_string(),
            user_id: user.id.to_string(),
            email: user.email,
        })
    }

    pub async fn login_user(&self, request: LoginRequest) -> Result<LoginResponse, CustomError> {
        let user = self
            .user_repo
            .get_user_by_email(request.email)
            .map_err(|_| CustomError::InvalidCredentials)
            .await?;

        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(
            user.password_hash
                .as_deref()
                .ok_or(CustomError::InvalidCredentials)?,
        )
        .map_err(|_| CustomError::InvalidCredentials)?;

        if let Err(_) = argon2
            .verify_password(request.password.as_bytes(), &parsed_hash)
            .map_err(|_| CustomError::InvalidCredentials)
        {
            return Err(CustomError::InvalidCredentials);
        }

        let (access_token, refresh_token, access_token_exp, refresh_token_exp) = self
            .token_service
            .generate_token_pair(user.id.clone())
            .await?;

        self.user_repo
            .update_user_last_login(user.id)
            .map_err(|_| CustomError::DatabaseError("Failed to update last login".to_string()))
            .await?;

        Ok(LoginResponse {
            message: "User logged in successfully".to_string(),
            user_id: user.id.to_string(),
            email: user.email,
            access_token,
            refresh_token,
            expires_in_access: access_token_exp * 60,
            expires_in_refresh: refresh_token_exp * 60,
        })
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<LoginResponse, CustomError> {
        let user_id_str = self
            .token_service
            .verify_refresh_token(&refresh_token)
            .await?;

        let user_id = Uuid::parse_str(&user_id_str)
            .map_err(|_| CustomError::InvalidToken("Invalid user ID format".to_string()))?;

        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .map_err(|_| CustomError::InvalidToken("User not found".to_string()))
            .await?;

        let (access_token, refresh_token, access_token_exp, refresh_token_exp) =
            self.token_service.generate_token_pair(user_id).await?;

        Ok(LoginResponse {
            message: "Token refreshed successfully".to_string(),
            user_id: user.id.to_string(),
            email: user.email,
            access_token,
            refresh_token,
            expires_in_access: access_token_exp * 60,
            expires_in_refresh: refresh_token_exp * 60,
        })
    }

    pub async fn get_session(&self, user_id: Uuid) -> Result<User, CustomError> {
        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .map_err(|_| CustomError::InvalidToken("User not found".to_string()))
            .await?;

        Ok(user)
    }
}
