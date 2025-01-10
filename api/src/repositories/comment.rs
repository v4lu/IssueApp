use crate::models::comment::{Comment, CommentRequest, CommentType, UpdateCommentRequest};
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CommentRepository {
    pool: PgPool,
}

impl CommentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_comment(
        &self,
        data: CommentRequest,
        org_id: Uuid,
        creator_id: Uuid,
    ) -> Result<Comment, sqlx::Error> {
        let user = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (comment_type, content, parent_id, comment_owner_id, org_id, creator_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                org_id,
                creator_id,
                comment_type as "comment_type: _",
                comment_owner_id,
                content as "content: JsonValue",
                parent_id as "parent_id: _",
                edited_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            data.comment_type as CommentType as _,
            data.content,
            data.parent_id,
            data.comment_owner_id,
            org_id,
            creator_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_comment_by_id(&self, comment_id: Uuid) -> Result<Comment, sqlx::Error> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            SELECT
                id,
                org_id,
                creator_id,
                comment_type as "comment_type: _",
                comment_owner_id,
                content as "content: JsonValue",
                parent_id as "parent_id: _",
                edited_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM comments
            WHERE id = $1
            "#,
            comment_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    pub async fn get_comments_by_owner_id(
        &self,
        owner_id: Uuid,
    ) -> Result<Vec<Comment>, sqlx::Error> {
        let comments = sqlx::query_as!(
            Comment,
            r#"
            SELECT
                id,
                org_id,
                creator_id,
                comment_type as "comment_type: _",
                comment_owner_id,
                content as "content: JsonValue",
                parent_id,
                edited_at ,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM comments
            WHERE comment_owner_id = $1
            "#,
            owner_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(comments)
    }

    pub async fn update_comment(
        &self,
        comment_id: Uuid,
        data: UpdateCommentRequest,
    ) -> Result<Comment, sqlx::Error> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET content = $1
            WHERE id = $2
            RETURNING
                id,
                org_id,
                creator_id,
                comment_type as "comment_type: _",
                comment_owner_id,
                content as "content: JsonValue",
                parent_id as "parent_id: _",
                edited_at,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            data.content,
            comment_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    pub async fn delete_comment(&self, comment_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM comments
            WHERE id = $1
            "#,
            comment_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
