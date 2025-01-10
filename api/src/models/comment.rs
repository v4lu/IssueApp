use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::ValidationError;
use validator_derive::Validate;

use super::auth::User;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "comment_type", rename_all = "UPPERCASE")]
pub enum CommentType {
    Issue,
    Document,
    Attachment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub creator_id: Uuid,
    pub comment_type: CommentType,
    pub comment_owner_id: Uuid,
    pub content: serde_json::Value,
    pub parent_id: Option<Uuid>,
    pub edited_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- request/response models ---

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CommentRequest {
    pub comment_type: CommentType,
    #[validate(custom(function = "validate_content"))]
    pub content: serde_json::Value,
    pub parent_id: Option<Uuid>,
    pub comment_owner_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(custom(function = "validate_content"))]
    pub content: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub comment: Comment,
    pub creator: User,
}

// --- validation functions ---

fn validate_content(value: &serde_json::Value) -> Result<(), ValidationError> {
    if value.is_null() {
        return Err(ValidationError::new("Content cannot be empty"));
    }

    Ok(())
}
