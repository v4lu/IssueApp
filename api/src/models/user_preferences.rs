use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- data models ---

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub theme: String,
    pub language: String,
    pub default_org_id: Option<Uuid>,
    pub cta_color: String,
    pub cta_text_color: String,
    pub font_size: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// --- request/response models ---
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferenceRequest {
    pub user_id: Uuid,
}

// --- request/response models ---
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferenceUpdateRequest {
    pub default_org_id: Option<Uuid>,
    pub theme: String,
    pub language: String,
    pub cta_color: String,
    pub cta_text_color: String,
    pub font_size: String,
}
