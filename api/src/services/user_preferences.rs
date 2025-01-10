use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::user_preferences::{
        UserPreference, UserPreferenceRequest, UserPreferenceUpdateRequest,
    },
    repositories::user_preferences::UserPreferencesRepository,
};

pub struct UserPreferencesService {
    user_preferences_repo: UserPreferencesRepository,
}

impl UserPreferencesService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_preferences_repo: UserPreferencesRepository::new(pool.clone()),
        }
    }

    pub async fn create_user_preference(
        &self,
        data: UserPreferenceRequest,
    ) -> Result<UserPreference, sqlx::Error> {
        self.user_preferences_repo
            .create_user_preference(data)
            .await
    }

    pub async fn get_user_preferences(&self, user_id: Uuid) -> Result<UserPreference, sqlx::Error> {
        self.user_preferences_repo
            .get_user_preferences(user_id)
            .await
    }

    pub async fn update_user_preference(
        &self,
        user_id: Uuid,
        data: UserPreferenceUpdateRequest,
    ) -> Result<UserPreference, sqlx::Error> {
        self.user_preferences_repo
            .update_user_preferences(user_id, data)
            .await
    }
}
