use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::auth::{
    CreateGithubUser, CreateUserData, UpdateGithubUser, UpdateUserData, User,
};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, data: CreateUserData) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                email,
                username,
                password_hash,
                is_email_verified as "is_email_verified!: bool",
                email_verification_token,
                email_verification_expires_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                last_login_at,
                avatar_url,
                github_id,
                github_url
            "#,
            data.email,
            data.username,
            data.password,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                email,
                username,
                password_hash,
                is_email_verified as "is_email_verified!: bool",
                email_verification_token,
                email_verification_expires_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                last_login_at,
                avatar_url,
                github_id,
                github_url
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                email,
                username,
                password_hash,
                is_email_verified as "is_email_verified!: bool",
                email_verification_token,
                email_verification_expires_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                last_login_at,
                avatar_url,
                github_id,
                github_url
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user_last_login(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET last_login_at = $1
            WHERE id = $2
            "#,
            Utc::now(),
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_user_email_verification_status(
        &self,
        user_id: Uuid,
        is_email_verified: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET is_email_verified = $1
            WHERE id = $2
            "#,
            is_email_verified,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_user_email_verification_token(
        &self,
        user_id: Uuid,
        email_verification_token: &str,
        email_verification_expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET email_verification_token = $1,
                email_verification_expires_at = $2
            WHERE id = $3
            "#,
            email_verification_token,
            email_verification_expires_at,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_user(
        &self,
        user_data: UpdateUserData,
        user_id: Uuid,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $1,
                username = $2,
                password_hash = $3,
                updated_at = $4
            WHERE id = $5
            RETURNING
                id,
                email,
                username,
                password_hash,
                is_email_verified as "is_email_verified!: bool",
                email_verification_token,
                email_verification_expires_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                last_login_at,
                avatar_url,
                github_id,
                github_url
            "#,
            user_data.email,
            user_data.username,
            user_data.password,
            Utc::now(),
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn user_exists(&self, email: String, username: String) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE email = $1 OR username = $2
            )
            AS "exists!"
            "#,
            email,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.exists)
    }

    pub async fn insert_github_user(&self, data: CreateGithubUser) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username, github_id, github_url, avatar_url)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                email,
                username,
                password_hash,
                is_email_verified as "is_email_verified!: bool",
                email_verification_token,
                email_verification_expires_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                last_login_at,
                avatar_url,
                github_id,
                github_url
            "#,
            data.email,
            data.username,
            data.github_id,
            data.github_url,
            data.avatar_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_github_user(
        &self,
        user_id: Uuid,
        data: UpdateGithubUser,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $1,
                github_id = $2,
                github_url = $3,
                avatar_url = $4,
                updated_at = $5
            WHERE id = $6
            RETURNING
                id,
                email,
                username,
                password_hash,
                is_email_verified as "is_email_verified!: bool",
                email_verification_token,
                email_verification_expires_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                last_login_at,
                avatar_url,
                github_id,
                github_url
            "#,
            data.email,
            data.github_id,
            data.github_url,
            data.avatar_url,
            Utc::now(),
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_github_id(&self, github_id: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                email,
                username,
                password_hash,
                is_email_verified as "is_email_verified!: bool",
                email_verification_token,
                email_verification_expires_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                last_login_at,
                avatar_url,
                github_id,
                github_url
            FROM users
            WHERE github_id = $1
            "#,
            github_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
