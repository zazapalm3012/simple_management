use std::sync::Arc;
use anyhow::{Ok, Result};
use axum::async_trait;
use diesel::prelude::*;

use crate::{domain::{entities::tickets::TicketEntity, repositories::ticket_viewing::TicketViewingRepository}, infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::tickets}};

pub struct TicketViewingPostgres{
    db_pool: Arc<PgPoolSquad>
}

impl TicketViewingPostgres{
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self{
        Self { db_pool }
    } 
}


#[async_trait]
impl TicketViewingRepository for TicketViewingPostgres{
    async fn view(&self) -> Result<Vec<TicketEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result: Vec<TicketEntity> = tickets::table
        .select(TicketEntity::as_select())
        .order_by(tickets::created_at.desc())
        .load::<TicketEntity>(&mut conn)?;

        Ok(result)
    }
}