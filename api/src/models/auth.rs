use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

// --- data models ---

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: Option<String>,
    pub is_email_verified: bool,
    pub email_verification_token: Option<String>,
    pub email_verification_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub avatar_url: Option<String>,
    pub github_id: Option<String>,
    pub github_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub replaced_by_token: Option<Uuid>,
    pub device_info: Option<JsonValue>,
    pub ip_address: Option<String>,
    pub is_valid: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RevokedToken {
    pub id: Uuid,
    pub token_identifier: String,
    pub revoked_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revocation_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub details: Option<JsonValue>,
}

// --- request/response models ---

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
    pub user_id: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in_refresh: i64,
    pub expires_in_access: i64,
}

// --- repository models ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserData {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserData {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRefreshTokenData {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub device_info: Option<JsonValue>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRevokedTokenData {
    pub token_identifier: String,
    pub revoked_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revocation_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGithubUser {
    pub email: String,
    pub username: String,
    pub github_id: String,
    pub github_url: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGithubUser {
    pub email: Option<String>,
    pub github_id: Option<String>,
    pub github_url: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGithubResponse {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expiration: i64,
    pub refresh_token_expiration: i64,
}
