use uuid::Uuid;

use super::org::Org;

#[derive(Debug, Clone, Default)]
pub struct UserContext {
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Clone, Default)]
pub struct OrgContext {
    pub org: Option<Org>,
}
