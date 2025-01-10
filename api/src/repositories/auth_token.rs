use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::auth::{CreateRefreshTokenData, RefreshToken};

pub struct AuthTokenRepository {
    pool: PgPool,
}

impl AuthTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_refresh_token(
        &self,
        data: CreateRefreshTokenData,
    ) -> Result<RefreshToken, sqlx::Error> {
        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at, device_info, ip_address)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                user_id,
                token_hash,
                expires_at as "expires_at!: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                revoked_at as "revoked_at!: DateTime<Utc>",
                replaced_by_token,
                device_info,
                ip_address,
                is_valid as "is_valid!: bool"
            "#,
            data.user_id,
            data.token_hash,
            data.expires_at,
            data.device_info,
            data.ip_address
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    pub async fn get_refresh_token_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<RefreshToken, sqlx::Error> {
        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT
                id,
                user_id,
                token_hash,
                expires_at as "expires_at!: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                revoked_at as "revoked_at!: DateTime<Utc>",
                replaced_by_token,
                device_info,
                ip_address,
                is_valid as "is_valid!: bool"
            FROM refresh_tokens
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    pub async fn revoke_refresh_token(&self, token_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = now()
            WHERE id = $1
            "#,
            token_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn revoke_all_refresh_tokens_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = now()
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
