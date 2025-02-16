use serde::{Deserialize, Serialize};

use super::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Doc {
    pub id: uuid::Uuid,
    pub org_id: uuid::Uuid,
    pub creator_id: uuid::Uuid,
    pub number: i32,
    pub title: String,
    pub sharable: bool,
    pub sharable_link: Option<String>,
    pub content: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// === response / request ===
#[derive(Debug, Deserialize)]
pub struct DocRequest {
    pub title: String,
    pub content: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct DocResponse {
    pub doc: Doc,
    pub creator: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDocRequest {
    pub title: Option<String>,
    pub content: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct SharableLinkResponse {
    pub sharable_link: String,
}

#[derive(Debug, Serialize)]
pub struct DocPagable {
    pub docs: Vec<Doc>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub last_page: i64,
}

#[derive(Debug, Serialize)]
pub struct DocPagableResponse {
    pub docs: Vec<DocResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub last_page: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DocQueryParams {
    pub sort: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DocPageRequest {
    pub page: i64,
    pub page_size: i64,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

impl DocPageRequest {
    pub fn get_offset(&self) -> i64 {
        (self.page - 1) * self.page_size
    }
    pub fn get_order(&self) -> String {
        match self.order.as_deref() {
            Some("asc") => "ASC",
            Some("desc") => "DESC",
            _ => "ASC",
        }
        .to_string()
    }
    pub fn get_sort(&self) -> String {
        match self.sort.as_deref() {
            Some("created_at") => "created_at",
            Some("updated_at") => "updated_at",
            _ => "created_at",
        }
        .to_string()
    }
}
