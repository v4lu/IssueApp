use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::ValidationError;
use validator_derive::Validate;

use super::comment::CommentResponse;
// --- data models ---

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "issue_priority", rename_all = "UPPERCASE")]
pub enum IssuePriority {
    Urgent,
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "issue_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IssueStatus {
    Backlog,
    Todo,
    #[sqlx(rename = "IN_PROGRESS")]
    InProgress,
    #[sqlx(rename = "IN_REVIEW")]
    InReview,
    Done,
    Canceled,
    Blocked,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Issue {
    pub id: Uuid,
    pub org_id: Uuid,
    pub creator_id: Uuid,
    pub number: i32,
    pub title: String,
    pub description: Option<serde_json::Value>,
    pub priority: IssuePriority,
    pub status: IssueStatus,
    pub parent_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- request/response models ---

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct IssueRequest {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    pub description: Option<serde_json::Value>,

    #[validate(custom(function = "validate_priority"))]
    pub priority: IssuePriority,

    #[validate(custom(function = "validate_status"))]
    pub status: IssueStatus,
    pub parent_id: Option<Uuid>,

    #[validate(custom(function = "validate_due_date"))]
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateIssueRequest {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: Option<String>,
    pub description: Option<serde_json::Value>,
    #[validate(custom(function = "validate_priority"))]
    pub priority: Option<IssuePriority>,

    #[validate(custom(function = "validate_status"))]
    pub status: Option<IssueStatus>,
    pub parent_id: Option<Uuid>,

    #[validate(custom(function = "validate_due_date"))]
    pub due_date: Option<DateTime<Utc>>,

    pub remove_due_date: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueResponse {
    pub issue: Issue,
    pub issue_id: Uuid,
    pub sub_issues: Option<Vec<Issue>>,
    pub comments: Option<Vec<CommentResponse>>,
}

// --- validators funcs ---

fn validate_due_date(due_date: &DateTime<Utc>) -> Result<(), ValidationError> {
    if *due_date < Utc::now() {
        return Err(ValidationError::new("due_date cannot be in the past"));
    }
    Ok(())
}

// this function are not doing anything, if user provides wrong status or
// priority, it will return error (400 but not 422) and it is ugly formatted
fn validate_status(status: &IssueStatus) -> Result<(), ValidationError> {
    match status {
        IssueStatus::Backlog
        | IssueStatus::Todo
        | IssueStatus::InProgress
        | IssueStatus::InReview
        | IssueStatus::Done
        | IssueStatus::Canceled
        | IssueStatus::Blocked => Ok(()),
    }
}

fn validate_priority(priority: &IssuePriority) -> Result<(), ValidationError> {
    match priority {
        IssuePriority::Urgent
        | IssuePriority::High
        | IssuePriority::Medium
        | IssuePriority::Low => Ok(()),
    }
}
