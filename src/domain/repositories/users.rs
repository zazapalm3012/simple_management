use crate::domain::entities::users::{RegisterUserEntity, UserEntity};
use anyhow::Result;
use mockall::automock;
use axum::async_trait;

#[automock]
#[async_trait]
pub trait UserRepository{
    async fn register_user(&self, register_user_entity: RegisterUserEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<UserEntity>;
}