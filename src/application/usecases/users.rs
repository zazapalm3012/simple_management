use crate::domain::{repositories::users::UserRepository, value_object::user_model::RegisterUserModel};
use crate::infrastructure::argon2_hashing;
use std::sync::Arc;
use anyhow::{Ok, Result};

pub struct UserUseCase<T>
where
    T: UserRepository + Send + Sync
{
    users_repository: Arc<T>,
}

impl<T> UserUseCase<T>
where
    T: UserRepository + Send + Sync,
{
    pub fn new(users_repository: Arc<T>) -> Self{
        Self { users_repository }
    }

    pub async fn register(&self, mut register_user_model: RegisterUserModel) -> Result<i32>{
        let hashed_password = argon2_hashing::hash(register_user_model.password.clone())?;
        register_user_model.password = hashed_password;

        let register_entity = register_user_model.to_entity();
        let user_id = self.users_repository.register_user(register_entity).await?;
        Ok(user_id)
    }
    
}