use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::CustomError,
    models::{
        comment::CommentResponse,
        issue::{IssueRequest, IssueResponse, UpdateIssueRequest},
    },
    repositories::{comment::CommentRepository, issue::IssueRepository, user::UserRepository},
};

pub struct IssueService {
    issue_repo: IssueRepository,
    comment_repo: CommentRepository,
    user_repo: UserRepository,
}

impl IssueService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            issue_repo: IssueRepository::new(pool.clone()),
            user_repo: UserRepository::new(pool.clone()),
            comment_repo: CommentRepository::new(pool),
        }
    }

    pub async fn create_issue(
        &self,
        data: IssueRequest,
        org_id: Uuid,
        creator_id: Uuid,
    ) -> Result<IssueResponse, CustomError> {
        if let Err(validation_errors) = data.validate() {
            return Err(CustomError::ValidationError(validation_errors));
        }

        let issue = self
            .issue_repo
            .create_issue(data, org_id, creator_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        return Ok(IssueResponse {
            issue_id: issue.id.clone(),
            issue,
            sub_issues: None,
            comments: None,
        });
    }

    pub async fn get_issue(&self, id: Uuid) -> Result<IssueResponse, CustomError> {
        let issue = self
            .issue_repo
            .get_issue_by_id(id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    CustomError::NotFound(format!("Issue with id: {} not found", id))
                }
                _ => CustomError::DatabaseError(e.to_string()),
            })?;

        let sub_issues = self
            .issue_repo
            .get_sub_issues(id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        let comments = self
            .comment_repo
            .get_comments_by_owner_id(issue.id.clone())
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        let mut comments_response = Vec::new();

        for comment in comments {
            let user = self
                .user_repo
                .get_user_by_id(comment.creator_id)
                .await
                .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

            comments_response.push(CommentResponse {
                comment,
                creator: user,
            });
        }

        Ok(IssueResponse {
            issue_id: id,
            issue,
            sub_issues: Some(sub_issues),
            comments: Some(comments_response),
        })
    }

    pub async fn delete_issue(&self, id: Uuid) -> Result<(), CustomError> {
        self.issue_repo.delete_issue(id).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                CustomError::NotFound(format!("Issue with id: {} not found", id))
            }
            _ => CustomError::DatabaseError(e.to_string()),
        })
    }

    pub async fn update_issue(
        &self,
        id: Uuid,
        update_data: UpdateIssueRequest,
    ) -> Result<IssueResponse, CustomError> {
        if let Err(validation_errors) = update_data.validate() {
            return Err(CustomError::ValidationError(validation_errors));
        }

        let issue = self
            .issue_repo
            .update_issue(id, update_data)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    CustomError::NotFound(format!("Issue with id: {} not found", id))
                }
                _ => CustomError::DatabaseError(e.to_string()),
            })?;

        let sub_issues = self
            .issue_repo
            .get_sub_issues(id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        let comments = self
            .comment_repo
            .get_comments_by_owner_id(issue.id.clone())
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        let mut comments_response = Vec::new();

        for comment in comments {
            let user = self
                .user_repo
                .get_user_by_id(comment.creator_id)
                .await
                .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

            comments_response.push(CommentResponse {
                comment,
                creator: user,
            });
        }

        Ok(IssueResponse {
            issue_id: id,
            issue,
            sub_issues: Some(sub_issues),
            comments: Some(comments_response),
        })
    }

    pub async fn get_all_by_org_id(&self, org_id: Uuid) -> Result<Vec<IssueResponse>, CustomError> {
        let issues = self
            .issue_repo
            .get_all_issues_by_org_id(org_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()));

        let mut issue_responses = Vec::new();

        for issue in issues? {
            let sub_issues = self
                .issue_repo
                .get_sub_issues(issue.id.clone())
                .await
                .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

            let comments = self
                .comment_repo
                .get_comments_by_owner_id(issue.id.clone())
                .await
                .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

            let mut comments_response = Vec::new();

            for comment in comments {
                let user = self
                    .user_repo
                    .get_user_by_id(comment.creator_id)
                    .await
                    .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

                comments_response.push(CommentResponse {
                    comment,
                    creator: user,
                });
            }

            issue_responses.push(IssueResponse {
                issue_id: issue.id.clone(),
                issue,
                comments: Some(comments_response),
                sub_issues: Some(sub_issues),
            });
        }

        Ok(issue_responses)
    }
}
