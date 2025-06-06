use crate::domain::entities::tickets::{ AddTicketEntity, EditTicketEntity };
use anyhow::Result;
use mockall::automock;
use axum::async_trait;

#[automock]
#[async_trait]
pub trait TicketOpsRepository{
    async fn add(&self, add_user_entity: AddTicketEntity) -> Result<i32>;
    async fn remove(&self, ticket_id: i32) -> Result<i32>;
    async fn edit(&self, ticket_id: i32, edit_ticket_entity: EditTicketEntity) -> Result<i32>;
}

