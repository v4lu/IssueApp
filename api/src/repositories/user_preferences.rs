use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user_preferences::{
    UserPreference, UserPreferenceRequest, UserPreferenceUpdateRequest,
};

pub struct UserPreferencesRepository {
    pool: PgPool,
}

impl UserPreferencesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user_preference(
        &self,
        data: UserPreferenceRequest,
    ) -> Result<UserPreference, sqlx::Error> {
        let user_preference = sqlx::query_as!(
            UserPreference,
            r#"
            INSERT INTO user_preferences (user_id)
            VALUES ($1)
            RETURNING
                id,
                user_id,
                theme,
                language,
                default_org_id,
                cta_color,
                cta_text_color,
                font_size,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            data.user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user_preference)
    }

    pub async fn get_user_preferences(&self, user_id: Uuid) -> Result<UserPreference, sqlx::Error> {
        let result = sqlx::query_as!(
            UserPreference,
            r#"
            SELECT
                id,
                user_id,
                theme,
                language,
                default_org_id,
                cta_color,
                cta_text_color,
                font_size,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM user_preferences
            WHERE user_id = $1

            "#,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        if result.user_id != user_id {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(result)
    }

    pub async fn update_user_preferences(
        &self,
        id: Uuid,
        user_preferences: UserPreferenceUpdateRequest,
    ) -> Result<UserPreference, sqlx::Error> {
        let result = sqlx::query_as!(
            UserPreference,
            r#"
            UPDATE user_preferences
            SET
                theme = $2,
                language = $3,
                default_org_id = $4,
                cta_color = $5,
                cta_text_color = $6,
                font_size = $7,
                updated_at = $8
            WHERE user_id = $1
            RETURNING
                id,
                user_id,
                theme,
                language,
                default_org_id,
                cta_color,
                cta_text_color,
                font_size,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            id,
            user_preferences.theme,
            user_preferences.language,
            user_preferences.default_org_id,
            user_preferences.cta_color,
            user_preferences.cta_text_color,
            user_preferences.font_size,
            Utc::now(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}
