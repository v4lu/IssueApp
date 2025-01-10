/**
 *
 *
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
    #[serde(flatten)]
    pub comment: Comment,
    pub creator: User,
}
 */

import type { User } from './user.type';

type CommentType = 'Issue' | 'Document' | 'Attachment';

export type Comment = {
	id: string;
	org_id: string;
	creator_id: string;
	comment_type: CommentType;
	comment_owner_id: string;
	content: string;
	parent_id: string | null;
	edited_at: string | null;
	created_at: string;
	updated_at: string;
};

export type CommentRequest = {
	comment_type: CommentType;
	content: string;
	parent_id?: string;
	comment_owner_id: string;
};

export type UpdateCommentRequest = {
	content: string;
};

export type CommentResponse = {
	comment: Comment;
	creator: User;
};
