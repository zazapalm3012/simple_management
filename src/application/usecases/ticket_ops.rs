use crate::domain::{repositories::ticket_ops::TicketOpsRepository};
use crate::domain::value_object::ticket_model::{AddTicketModel, EditTicketModel};
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

    pub async fn edit(&self, ticket_id: i32, edit_ticket_entity: EditTicketModel) -> Result<i32>{
        let edit_ticket_entity = edit_ticket_entity.to_entity();

        let ticket_id = self.ticket_ops_repository.edit(ticket_id, edit_ticket_entity).await?;
        Ok(ticket_id)
    }
}