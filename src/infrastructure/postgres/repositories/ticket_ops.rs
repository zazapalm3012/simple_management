use std::sync::Arc;
use anyhow::{Ok, Result};
use axum::async_trait;
use diesel::{dsl::insert_into, prelude::*};

use crate::{domain::{entities::tickets::AddTicketEntity, repositories::ticket_ops::TicketOpsRepository}, infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::tickets}};

pub struct TicketOpsPostgres{
    db_pool: Arc<PgPoolSquad>
}

impl TicketOpsPostgres{
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self{
        Self { db_pool }
    } 
}

#[async_trait]
impl TicketOpsRepository for TicketOpsPostgres{
    async fn add(&self, add_user_entity: AddTicketEntity) -> Result<i32>{
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(tickets::table)
        .values(add_user_entity)
        .returning(tickets::id)
        .get_result::<i32>(&mut conn)?;

        Ok(result)
    }
}