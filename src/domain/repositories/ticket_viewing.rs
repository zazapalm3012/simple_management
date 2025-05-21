use crate::domain::entities::tickets::{ TicketEntity };
use anyhow::Result;
use mockall::automock;
use axum::async_trait;

#[automock]
#[async_trait]
pub trait TicketViewingRepository{
    async fn view(&self) -> Result<Vec<TicketEntity>>;
}