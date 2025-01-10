use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    auth::User,
    org::{
        CreateOrgRequest, MemberRole, MemberStatus, Org, OrgMember, OrgMemberInvite,
        OrgMemberInviteResponse, OrgMemberResponse, UpdateOrgRequest,
    },
};

pub struct OrgRepository {
    pool: PgPool,
}

impl OrgRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_org(
        &self,
        data: CreateOrgRequest,
        slug: String,
        custom_id: String,
    ) -> Result<Org, sqlx::Error> {
        let result = sqlx::query_as!(
            Org,
            r#"
          INSERT INTO org (name ,logo_url, slug, custom_id)
          VALUES ($1, $2, $3, $4)
          RETURNING
              id,
              name,
              slug,
              custom_id,
              created_at as "created_at!: DateTime<Utc>",
              updated_at as "updated_at!: DateTime<Utc>",
              logo_url
          "#,
            data.name,
            data.logo_url,
            slug,
            custom_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Org, sqlx::Error> {
        let result = sqlx::query_as!(
            Org,
            r#"
            SELECT
                o.id,
                o.name,
                o.slug,
                o.custom_id,
                o.created_at as "created_at!: DateTime<Utc>",
                o.updated_at as "updated_at!: DateTime<Utc>",
                o.logo_url
            FROM org o
            INNER JOIN org_members om
                ON o.id = om.org_id
                AND om.user_id = $2
                AND om.status = 'ACTIVE'
            WHERE o.id = $1
            "#,
            id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        if result.id != id {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(result)
    }

    pub async fn public_find_by_slug(&self, slug: String) -> Result<Org, sqlx::Error> {
        let result = sqlx::query_as!(
            Org,
            r#"
            SELECT
                id,
                name,
                slug,
                custom_id,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                logo_url
            FROM org
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        Ok(result)
    }

    pub async fn find_by_slug(&self, slug: String, user_id: Uuid) -> Result<Org, sqlx::Error> {
        let result = sqlx::query_as!(
            Org,
            r#"
            SELECT
                o.id,
                o.name,
                o.slug,
                o.custom_id,
                o.created_at as "created_at!: DateTime<Utc>",
                o.updated_at as "updated_at!: DateTime<Utc>",
                o.logo_url
            FROM org o
            INNER JOIN org_members om
                ON o.id = om.org_id
                AND om.user_id = $2
                AND om.status = 'ACTIVE'
            WHERE o.slug = $1
            "#,
            slug,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        if result.slug != slug {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(result)
    }

    pub async fn list_user_orgs(&self, user_id: Uuid) -> Result<Vec<Org>, sqlx::Error> {
        let result = sqlx::query_as!(
            Org,
            r#"
            SELECT
                o.id,
                o.name,
                o.slug,
                o.custom_id,
                o.created_at as "created_at!: DateTime<Utc>",
                o.updated_at as "updated_at!: DateTime<Utc>",
                o.logo_url
            FROM org o
            INNER JOIN org_members om
                ON o.id = om.org_id
                AND om.user_id = $1
                AND om.status = 'ACTIVE'
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn update_org(&self, id: Uuid, data: UpdateOrgRequest) -> Result<Org, sqlx::Error> {
        let result = sqlx::query_as!(
            Org,
            r#"
            UPDATE org
            SET name = $2,
                logo_url = $3,
                updated_at = $4
            WHERE id = $1
            RETURNING
                id,
                name,
                slug,
                custom_id,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>",
                logo_url
            "#,
            id,
            data.name,
            data.logo_url,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        if result.id != id {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(result)
    }

    pub async fn delete_org(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM org WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_member(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        role: MemberRole,
        invited_by: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO org_members (org_id, user_id, role, status, invited_by)
            VALUES ($1, $2, $3::member_role, 'ACTIVE', $4)
            "#,
            org_id,
            user_id,
            role as MemberRole,
            invited_by
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_org_invites(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<OrgMemberInvite>, sqlx::Error> {
        let results = sqlx::query_as!(
            OrgMemberInvite,
            r#"
        SELECT
            id,
            org_id,
            email,
            role as "role: MemberRole",
            invited_by,
            invited_at as "invited_at!: DateTime<Utc>",
            expires_at as "expires_at!: DateTime<Utc>",
            token
        FROM org_invites
        WHERE org_id = $1 AND expires_at > NOW()
        "#,
            org_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    pub async fn update_member_role(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        role: MemberRole,
    ) -> Result<(), sqlx::Error> {
        let role_str = match role {
            MemberRole::Owner => "OWNER",
            MemberRole::Admin => "ADMIN",
            MemberRole::Member => "MEMBER",
        };

        let result = sqlx::query!(
            r#"
          UPDATE org_members
          SET role = $3::member_role
          WHERE org_id = $1 AND user_id = $2
          "#,
            org_id,
            user_id,
            role_str as &str
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn remove_member(&self, org_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            r#"
          DELETE FROM org_members
          WHERE org_id = $1 AND user_id = $2
          "#,
            org_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn list_members(&self, org_id: Uuid) -> Result<Vec<OrgMemberResponse>, sqlx::Error> {
        let results = sqlx::query!(
            r#"
            SELECT
                u.id as "user_id",
                u.email as "user_email",
                u.username as "user_username",
                u.password_hash as "user_password_hash",
                u.is_email_verified as "user_is_email_verified!: bool",
                u.email_verification_token as "user_email_verification_token",
                u.email_verification_expires_at as "user_email_verification_expires_at",
                u.created_at as "user_created_at!: DateTime<Utc>",
                u.updated_at as "user_updated_at!: DateTime<Utc>",
                u.last_login_at as "user_last_login_at",
                u.avatar_url as "user_avatar_url",
                u.github_id as "user_github_id",
                u.github_url as "user_github_url",
                om.id as "org_member_id",
                om.org_id as "org_member_org_id",
                om.user_id as "org_member_user_id",
                om.role as "org_member_role: MemberRole",
                om.status as "org_member_status: MemberStatus",
                om.joined_at as "org_member_joined_at!: DateTime<Utc>",
                om.invited_by as "org_member_invited_by",
                om.created_at as "org_member_created_at!: DateTime<Utc>",
                om.updated_at as "org_member_updated_at!: DateTime<Utc>"
            FROM org_members om
            INNER JOIN users u
                ON om.user_id = u.id
            WHERE om.org_id = $1
            "#,
            org_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut org_members = Vec::new();

        for row in results {
            org_members.push(OrgMemberResponse {
                user: User {
                    id: row.user_id,
                    email: row.user_email,
                    username: row.user_username,
                    password_hash: row.user_password_hash,
                    is_email_verified: row.user_is_email_verified,
                    email_verification_token: row.user_email_verification_token,
                    email_verification_expires_at: row.user_email_verification_expires_at,
                    created_at: row.user_created_at,
                    updated_at: row.user_updated_at,
                    last_login_at: row.user_last_login_at,
                    avatar_url: row.user_avatar_url,
                    github_id: row.user_github_id,
                    github_url: row.user_github_url,
                },
                org_member: OrgMember {
                    id: row.org_member_id,
                    org_id: row.org_member_org_id,
                    user_id: row.org_member_user_id,
                    role: row.org_member_role,
                    status: row.org_member_status,
                    joined_at: row.org_member_joined_at,
                    invited_by: row.org_member_invited_by,
                    created_at: row.org_member_created_at,
                    updated_at: row.org_member_updated_at,
                },
            });
        }

        Ok(org_members)
    }

    pub async fn check_user_role(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<MemberRole, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT role as "role: MemberRole"
            FROM org_members
            WHERE org_id = $1
            AND user_id = $2
            AND status = 'ACTIVE'
            "#,
            org_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        Ok(result.role)
    }

    pub async fn create_invite(
        &self,
        org_id: Uuid,
        email: String,
        role: MemberRole,
        invited_by: Uuid,
    ) -> Result<OrgMemberInvite, sqlx::Error> {
        let expires_at = Utc::now() + chrono::Duration::days(7);
        let token = uuid::Uuid::new_v4().to_string();

        let result = sqlx::query_as!(
            OrgMemberInvite,
            r#"
          INSERT INTO org_invites (
              org_id, email, role, invited_by, expires_at, token
          )
          VALUES ($1, $2, $3, $4, $5, $6)
          RETURNING
              id,
              org_id,
              email,
              role as "role: MemberRole",
              invited_by,
              invited_at as "invited_at!: DateTime<Utc>",
              expires_at as "expires_at!: DateTime<Utc>",
              token
          "#,
            org_id,
            email,
            role as MemberRole,
            invited_by,
            expires_at,
            token
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn list_user_invites(
        &self,
        email: String,
    ) -> Result<Vec<OrgMemberInviteResponse>, sqlx::Error> {
        let results = sqlx::query!(
            r#"
            SELECT
            oi.id as "invite_id!",
            oi.org_id as "invite_org_id!",
            oi.email as "invite_email!",
            oi.role as "invite_role!: MemberRole",
            oi.invited_by as "invite_invited_by!",
            oi.invited_at as "invite_invited_at!: DateTime<Utc>",
            oi.expires_at as "invite_expires_at!: DateTime<Utc>",
            oi.token as "invite_token!",
            o.name as "org_name!",
            o.logo_url as "org_logo",
            u.id as "user_id!",
            u.email as "user_email!",
            u.username as "user_username!",
            u.password_hash as "user_password_hash!",
            u.avatar_url as "user_avatar_url",
            u.github_id as "user_github_id",
            u.github_url as "user_github_url",
            u.is_email_verified as "user_is_email_verified!: bool",
            u.email_verification_token as "user_email_verification_token",
            u.email_verification_expires_at as "user_email_verification_expires_at",
            u.created_at as "user_created_at!: DateTime<Utc>",
            u.updated_at as "user_updated_at!: DateTime<Utc>",
            u.last_login_at as "user_last_login_at"
            FROM org_invites oi
            INNER JOIN org o ON oi.org_id = o.id
            INNER JOIN users u ON oi.invited_by = u.id
            WHERE oi.email = $1 AND oi.expires_at > NOW()
            "#,
            email
        )
        .fetch_all(&self.pool)
        .await?;

        let mut invites = Vec::new();

        for row in results {
            invites.push(OrgMemberInviteResponse {
                org_member_invite: OrgMemberInvite {
                    id: row.invite_id,
                    org_id: row.invite_org_id,
                    email: row.invite_email,
                    role: row.invite_role,
                    invited_by: row.invite_invited_by,
                    invited_at: row.invite_invited_at,
                    expires_at: row.invite_expires_at,
                    token: row.invite_token,
                },
                org_name: row.org_name,
                org_logo: row.org_logo,
                invited_by: User {
                    id: row.user_id,
                    email: row.user_email,
                    username: row.user_username,
                    password_hash: Some(row.user_password_hash),
                    is_email_verified: row.user_is_email_verified,
                    email_verification_token: row.user_email_verification_token,
                    email_verification_expires_at: row.user_email_verification_expires_at,
                    created_at: row.user_created_at,
                    updated_at: row.user_updated_at,
                    last_login_at: row.user_last_login_at,
                    avatar_url: row.user_avatar_url,
                    github_id: row.user_github_id,
                    github_url: row.user_github_url,
                },
            });
        }

        Ok(invites)
    }

    pub async fn remove_invite(&self, token: String) -> Result<(), sqlx::Error> {
        let result = sqlx::query!("DELETE FROM org_invites WHERE token = $1", token)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn find_invite_by_token(
        &self,
        token: String,
    ) -> Result<OrgMemberInvite, sqlx::Error> {
        let result = sqlx::query_as!(
            OrgMemberInvite,
            r#"
          SELECT
              id,
              org_id,
              email,
              role as "role: MemberRole",
              invited_by,
              invited_at as "invited_at!: DateTime<Utc>",
              expires_at as "expires_at!: DateTime<Utc>",
              token
          FROM org_invites
          WHERE token = $1
          "#,
            token
        )
        .fetch_one(&self.pool)
        .await?;

        if result.token != token {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(result)
    }

    pub async fn is_user_member_of_org(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
          SELECT EXISTS (
              SELECT 1
              FROM org_members
              WHERE org_id = $1 AND user_id = $2 AND status = 'ACTIVE'
          ) as "exists?"
          "#,
            org_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }
}
