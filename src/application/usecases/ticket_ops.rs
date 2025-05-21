use crate::domain::repositories::ticket_ops::TicketOpsRepository;
use crate::domain::value_object::ticket_model::AddTicketModel;
use std::sync::Arc;
use anyhow::{Ok, Result};

pub struct TicketOpsUsecase<T>
where
    T: TicketOpsRepository + Send + Sync
{
    ticket_ops_repository: Arc<T>
}

impl<T> TicketOpsUsecase<T>
where
    T: TicketOpsRepository + Send + Sync
{
    pub fn new(ticket_ops_repository: Arc<T>) -> Self{
        Self { ticket_ops_repository }
    }

    pub async fn add(&self, add_ticket_model: AddTicketModel) -> Result<i32>{
        let add_ticket_entity = add_ticket_model.to_entity();

        let ticket_id = self.ticket_ops_repository.add(add_ticket_entity).await?;

        Ok(ticket_id)
    }
    pub async fn remove(&self, ticket_id: i32) -> Result<i32>{
        let ticket_id = self.ticket_ops_repository.remove(ticket_id).await?;

        Ok(ticket_id)
    }
}