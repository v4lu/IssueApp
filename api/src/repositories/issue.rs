use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::issue::*;
use serde_json::Value as JsonValue;

pub struct IssueRepository {
    pool: PgPool,
}

impl IssueRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_issue(
        &self,
        new_issue: IssueRequest,
        org_id: Uuid,
        creator_id: Uuid,
    ) -> Result<Issue, sqlx::Error> {
        let issue = sqlx::query_as!(
            Issue,
            r#"
            WITH new_issue AS (
                SELECT COALESCE(MAX(number), 0) + 1 as next_number
                FROM issues
                WHERE org_id = $1
            )
            INSERT INTO issues (
                org_id, creator_id, number, title, description,
                priority, status, parent_id, due_date
            )
            SELECT
                $1, $2, next_number, $3, $4,
                $5::issue_priority, $6::issue_status, $7, $8
            FROM new_issue
            RETURNING
                id, org_id, creator_id, number,
                title, description as "description: JsonValue",
                priority as "priority: _",
                status as "status: _",
                parent_id,
                due_date,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            org_id,
            creator_id,
            new_issue.title,
            new_issue.description,
            new_issue.priority as IssuePriority as _,
            new_issue.status as IssueStatus as _,
            new_issue.parent_id,
            new_issue.due_date,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(issue)
    }

    pub async fn get_issue_by_id(&self, issue_id: Uuid) -> Result<Issue, sqlx::Error> {
        let issue = sqlx::query_as!(
            Issue,
            r#"
            SELECT
                id, org_id, creator_id, number,
                title, description as "description: JsonValue",
                priority as "priority: _",
                status as "status: _",
                parent_id,
                due_date,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM issues
            WHERE id = $1
            "#,
            issue_id,
        )
        .fetch_one(&self.pool)
        .await?;

        if issue.id != issue_id {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(issue)
    }

    pub async fn update_issue(
        &self,
        issue_id: Uuid,
        data: UpdateIssueRequest,
    ) -> Result<Issue, sqlx::Error> {
        let issue = sqlx::query_as!(
            Issue,
            r#"
        UPDATE issues
        SET
            title = COALESCE($2, title),
            description = COALESCE($3, description),
            priority = COALESCE($4::issue_priority, priority),
            status = COALESCE($5::issue_status, status),
            parent_id = COALESCE($6, parent_id),
            due_date = CASE
                WHEN $7 = true THEN NULL  -- When remove_due_date is true, set to NULL
                WHEN $8::timestamptz IS NOT NULL THEN $8::timestamptz  -- When new date provided
                ELSE due_date  -- Keep existing value
            END
        WHERE id = $1
        RETURNING
            id, org_id, creator_id, number,
            title, description as "description: JsonValue",
            priority as "priority: _",
            status as "status: _",
            parent_id,
            due_date,
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>"
        "#,
            issue_id,
            data.title,
            data.description,
            data.priority as Option<IssuePriority> as _,
            data.status as Option<IssueStatus> as _,
            data.parent_id,
            data.remove_due_date.unwrap_or(false),
            data.due_date,
        )
        .fetch_one(&self.pool)
        .await?;

        if issue.id != issue_id {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(issue)
    }

    pub async fn delete_issue(&self, issue_id: Uuid) -> Result<(), sqlx::Error> {
        let result = sqlx::query_as!(
            Issue,
            r#"
            DELETE FROM issues
            WHERE id = $1
            RETURNING
                id, org_id, creator_id, number,
                title, description as "description: JsonValue",
                priority as "priority: _",
                status as "status: _",
                parent_id,
                due_date,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            issue_id,
        )
        .fetch_one(&self.pool)
        .await?;

        if result.id != issue_id {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn get_all_issues_by_org_id(&self, org_id: Uuid) -> Result<Vec<Issue>, sqlx::Error> {
        let issues = sqlx::query_as!(
            Issue,
            r#"
            SELECT
                id, org_id, creator_id, number,
                title, description as "description: JsonValue",
                priority as "priority: _",
                status as "status: _",
                parent_id,
                due_date,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM issues
            WHERE org_id = $1 AND parent_id IS NULL
            "#,
            org_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(issues)
    }

    pub async fn get_sub_issues(&self, issue_id: Uuid) -> Result<Vec<Issue>, sqlx::Error> {
        let issues = sqlx::query_as!(
            Issue,
            r#"
            SELECT
                id, org_id, creator_id, number,
                title, description as "description: JsonValue",
                priority as "priority: _",
                status as "status: _",
                parent_id,
                due_date,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM issues
            WHERE parent_id = $1
            "#,
            issue_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(issues)
    }
}
