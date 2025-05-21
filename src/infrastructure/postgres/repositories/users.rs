use std::sync::Arc;
use anyhow::{Ok, Result};
use axum::async_trait;
use diesel::{dsl::insert_into, prelude::*};

use crate::domain::entities::users::{RegisterUserEntity, UserEntity};
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::domain::repositories::users::UserRepository;
use crate::infrastructure::postgres::schema::users;


pub struct UsersPostgres
{
    db_pool : Arc<PgPoolSquad>,
}

impl UsersPostgres{
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self{
        Self {db_pool}
    }
}

#[async_trait]
impl UserRepository for UsersPostgres{
    async fn register_user(&self, register_user_entity: RegisterUserEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(users::table)
        .values(&register_user_entity)
        .returning(users::id)
        .get_result::<i32>(&mut conn)?;
        
        Ok(result)
    }
    async fn find_by_username(&self, username: String) -> Result<UserEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = users::table
        .filter(users::username.eq(username))
        .select(UserEntity::as_select())
        .first::<UserEntity>(&mut conn)?;
        
        Ok(result)
    }
}