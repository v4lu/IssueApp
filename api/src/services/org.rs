use futures::TryFutureExt;
use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::CustomError,
    models::{
        org::{
            CreateOrgRequest, InviteOrgMemberRequest, MemberRole, Org, OrgMemberInvite,
            OrgMemberInviteResponse, OrgMemberResponse, UpdateOrgRequest,
        },
        user_preferences::UserPreferenceUpdateRequest,
    },
    repositories::{
        org::OrgRepository, user::UserRepository, user_preferences::UserPreferencesRepository,
    },
};

pub struct OrgService {
    user_repo: UserRepository,
    org_repo: OrgRepository,
    user_preferences_repo: UserPreferencesRepository,
}

impl OrgService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repo: UserRepository::new(pool.clone()),
            org_repo: OrgRepository::new(pool.clone()),
            user_preferences_repo: UserPreferencesRepository::new(pool.clone()),
        }
    }

    pub async fn create_org(
        &self,
        data: CreateOrgRequest,
        user_id: Uuid,
    ) -> Result<Org, CustomError> {
        if let Err(validation_errors) = data.validate() {
            return Err(CustomError::ValidationError(validation_errors));
        }

        let slug = self.generate_slug(&data.name).await;
        let custom_id = self.generate_custom_id(&data.name).await;
        let org = self
            .org_repo
            .create_org(data, slug, custom_id)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        self.org_repo
            .create_member(org.id, user_id, MemberRole::Owner, user_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        let user_preferences = self
            .user_preferences_repo
            .get_user_preferences(user_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        let user_preferences_update = UserPreferenceUpdateRequest {
            default_org_id: Some(org.id),
            theme: user_preferences.theme,
            language: user_preferences.language,
            cta_color: user_preferences.cta_color,
            cta_text_color: user_preferences.cta_text_color,
            font_size: user_preferences.font_size,
        };

        self.user_preferences_repo
            .update_user_preferences(user_id, user_preferences_update)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        Ok(org)
    }

    pub async fn get_org(&self, id: Uuid, user_id: Uuid) -> Result<Org, CustomError> {
        self.org_repo
            .find_by_id(id, user_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    CustomError::NotFound(format!("Org with id: {} not found", id))
                }
                _ => CustomError::DatabaseError(e.to_string()),
            })
    }

    pub async fn list_user_orgs(&self, user_id: Uuid) -> Result<Vec<Org>, CustomError> {
        self.org_repo
            .list_user_orgs(user_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
    }

    pub async fn update_org(&self, id: Uuid, data: UpdateOrgRequest) -> Result<Org, CustomError> {
        if let Err(validation_errors) = data.validate() {
            return Err(CustomError::ValidationError(validation_errors));
        }

        self.org_repo
            .update_org(id, data)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    CustomError::NotFound(format!("Org with id: {} not found", id))
                }
                _ => CustomError::DatabaseError(e.to_string()),
            })
    }

    pub async fn delete_org(&self, id: Uuid) -> Result<(), CustomError> {
        self.org_repo.delete_org(id).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                CustomError::NotFound(format!("Org with id: {} not found", id))
            }
            _ => CustomError::DatabaseError(e.to_string()),
        })
    }

    pub async fn invite_user_to_org(
        &self,
        org_id: Uuid,
        data: InviteOrgMemberRequest,
        role: MemberRole,
        invited_by: Uuid,
    ) -> Result<OrgMemberInvite, CustomError> {
        if let Err(validation_errors) = data.validate() {
            return Err(CustomError::ValidationError(validation_errors));
        }

        if self
            .user_repo
            .get_user_by_email(data.email.clone())
            .await
            .is_err()
        {
            return Err(CustomError::NotFound("User not found".to_string()));
        }

        self.org_repo
            .create_invite(org_id, data.email, role, invited_by)
            .await
            .map_err(|e: sqlx::Error| CustomError::DatabaseError(e.to_string()))
    }

    pub async fn accept_org_invite(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        token: String,
    ) -> Result<(), CustomError> {
        let invite = self
            .org_repo
            .find_invite_by_token(token.clone())
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => CustomError::NotFound("Invite not found".to_string()),
                _ => CustomError::DatabaseError(e.to_string()),
            })?;

        self.org_repo
            .create_member(org_id, user_id, MemberRole::Member, invite.invited_by)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?;

        self.org_repo
            .remove_invite(token)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
    }

    pub async fn decline_org_invite(&self, token: String) -> Result<(), CustomError> {
        self.org_repo
            .remove_invite(token)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => CustomError::NotFound("Invite not found".to_string()),
                _ => CustomError::DatabaseError(e.to_string()),
            })
    }

    pub async fn list_org_members(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<OrgMemberResponse>, CustomError> {
        self.org_repo
            .list_members(org_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
    }

    pub async fn list_org_invites(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<OrgMemberInvite>, CustomError> {
        self.org_repo
            .list_org_invites(org_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
    }

    pub async fn change_member_role(
        &self,
        org_id: Uuid,
        member_id: Uuid,
        role: MemberRole,
    ) -> Result<(), CustomError> {
        if !self
            .org_repo
            .is_user_member_of_org(org_id, member_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?
        {
            return Err(CustomError::NotFound("Member not found".to_string()));
        }

        self.org_repo
            .update_member_role(org_id, member_id, role)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
    }

    pub async fn remove_member(&self, org_id: Uuid, member_id: Uuid) -> Result<(), CustomError> {
        if !self
            .org_repo
            .is_user_member_of_org(org_id, member_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))?
        {
            return Err(CustomError::NotFound("Member not found".to_string()));
        }

        self.org_repo
            .remove_member(org_id, member_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
    }

    // i guess saving user id in the invite table is better solution
    pub async fn list_user_invites(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<OrgMemberInviteResponse>, CustomError> {
        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
            .await?;

        self.org_repo
            .list_user_invites(user.email)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
    }

    pub async fn check_user_role(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<MemberRole, CustomError> {
        self.org_repo
            .check_user_role(org_id, user_id)
            .await
            .map_err(|e| CustomError::DatabaseError(e.to_string()))
    }

    // temp solution
    async fn generate_slug(&self, name: &str) -> String {
        let sanitized = name
            .to_lowercase()
            .chars()
            .map(|c| match c {
                'a'..='z' | '0'..='9' => c,
                ' ' | '-' | '_' => '-',
                _ => '-',
            })
            .collect::<String>();

        let base_slug = sanitized
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");

        for _ in 0..10 {
            let random_numbers: u32 = rand::thread_rng().gen_range(100..1000);
            let new_slug = format!("{}-{}", base_slug, random_numbers);

            if self
                .org_repo
                .public_find_by_slug(new_slug.clone())
                .await
                .is_err()
            {
                return new_slug;
            }
        }

        format!("{}-{}", base_slug, uuid::Uuid::new_v4())
    }

    async fn generate_custom_id(&self, name: &str) -> String {
        name.to_uppercase().chars().take(3).collect::<String>()
    }
}
