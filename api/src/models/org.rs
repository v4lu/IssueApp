use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::ValidationError;
use validator_derive::Validate;

use super::auth::User;

// --- data models ---

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "member_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MemberRole {
    Owner,
    Admin,
    Member,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "member_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MemberStatus {
    Active,
    Invited,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Org {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub custom_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgMember {
    pub id: Uuid,
    pub org_id: Uuid,
    pub user_id: Uuid,
    pub role: MemberRole,
    pub status: MemberStatus,
    pub joined_at: DateTime<Utc>,
    pub invited_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgMemberInvite {
    pub id: Uuid,
    pub org_id: Uuid,
    pub email: String,
    pub role: MemberRole,
    pub invited_by: Uuid,
    pub invited_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub token: String,
}

// --- request/response models ---

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct CreateOrgRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(custom(function = "validate_logo_url"))]
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateOrgRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    #[validate(custom(function = "validate_logo_url"))]
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct InviteOrgMemberRequest {
    #[validate(custom(function = "validate_email"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrgMemberRequest {
    pub role: Option<MemberRole>,
    pub status: Option<MemberStatus>,
}

#[derive(Debug, Serialize)]
pub struct MembershipResponse {
    answer: bool,
}

#[derive(Debug, Serialize)]
pub struct OrgMemberResponse {
    pub user: User,
    pub org_member: OrgMember,
}

#[derive(Debug, Serialize)]
pub struct OrgMemberInviteResponse {
    pub org_member_invite: OrgMemberInvite,
    pub org_name: String,
    pub org_logo: Option<String>,
    pub invited_by: User,
}

// --- validation rules ---

fn validate_logo_url(url: &str) -> Result<(), validator::ValidationError> {
    if !url.starts_with("https://") {
        return Err(ValidationError::new("logo_url must be a valid URL"));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), validator::ValidationError> {
    let email_rgx = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_rgx.is_match(email) {
        return Err(ValidationError::new("email must be a valid email address"));
    }
    Ok(())
}
