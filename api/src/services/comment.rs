use futures::TryFutureExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::CustomError,
    models::comment::{CommentRequest, CommentResponse, UpdateCommentRequest},
    repositories::{comment::CommentRepository, user::UserRepository},
};

pub struct CommentService {
    comment_repo: CommentRepository,
    user_repo: UserRepository,
}

impl CommentService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            comment_repo: CommentRepository::new(pool.clone()),
            user_repo: UserRepository::new(pool),
        }
    }

    pub async fn create_comment(
        &self,
        data: CommentRequest,
        org_id: Uuid,
        creator_id: Uuid,
    ) -> Result<CommentResponse, CustomError> {
        let comment = self
            .comment_repo
            .create_comment(data, org_id, creator_id)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        let creator = self
            .user_repo
            .get_user_by_id(creator_id)
            .map_err(|_| CustomError::InvalidToken("User not found".to_string()))
            .await?;

        Ok(CommentResponse { comment, creator })
    }

    pub async fn delete_comment(&self, comment_id: Uuid, user_id: Uuid) -> Result<(), CustomError> {
        let comment = self
            .comment_repo
            .get_comment_by_id(comment_id)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        if comment.creator_id != user_id {
            return Err(CustomError::Unauthorized);
        }

        self.comment_repo
            .delete_comment(comment_id)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await
    }

    pub async fn update_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
        data: UpdateCommentRequest,
    ) -> Result<CommentResponse, CustomError> {
        let comment = self
            .comment_repo
            .get_comment_by_id(comment_id)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        if comment.creator_id != user_id {
            return Err(CustomError::Unauthorized);
        }

        let updated_comment = self
            .comment_repo
            .update_comment(comment_id, data)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        let creator = self
            .user_repo
            .get_user_by_id(comment.creator_id)
            .map_err(|_| CustomError::InvalidToken("User not found".to_string()))
            .await?;

        Ok(CommentResponse {
            comment: updated_comment,
            creator,
        })
    }

    pub async fn get_comment(&self, comment_id: Uuid) -> Result<CommentResponse, CustomError> {
        let comment = self
            .comment_repo
            .get_comment_by_id(comment_id)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        let creator = self
            .user_repo
            .get_user_by_id(comment.creator_id)
            .map_err(|_| CustomError::InvalidToken("User not found".to_string()))
            .await?;

        Ok(CommentResponse { comment, creator })
    }

    pub async fn get_comments_by_owner_id(
        &self,
        owner_id: Uuid,
    ) -> Result<Vec<CommentResponse>, CustomError> {
        let comments = self
            .comment_repo
            .get_comments_by_owner_id(owner_id)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        let mut comment_responses = Vec::new();
        for comment in comments {
            let creator = self
                .user_repo
                .get_user_by_id(comment.creator_id)
                .map_err(|_| CustomError::InvalidToken("User not found".to_string()))
                .await?;

            comment_responses.push(CommentResponse { comment, creator });
        }

        Ok(comment_responses)
    }
}
