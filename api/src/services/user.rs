use crate::{errors::CustomError, models::auth::User, repositories::user::UserRepository};

pub struct UserService {
   user_repo: UserRepository
}

impl UserService {
   pub fn new(pool: sqlx::PgPool) -> Self {
      Self {
         user_repo: UserRepository::new(pool.clone())
      }
   }

    pub async fn get_user(&self, user_id: uuid::Uuid) -> Result<User, CustomError> {
        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .await
            .map_err(|_| CustomError::InvalidToken("User not found".to_string()))?;

        Ok(user)
    }
}
