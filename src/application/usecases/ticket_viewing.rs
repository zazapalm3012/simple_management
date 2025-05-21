use std::sync::Arc;

use crate::domain::{entities::tickets::TicketEntity, repositories::ticket_viewing::TicketViewingRepository};

use anyhow::Result;

pub struct TicketViewingUseCase<T>
where 
    T: TicketViewingRepository + Send + Sync
{
    ticket_viewing_repo: Arc<T>
}

impl<T> TicketViewingUseCase<T>
where
    T: TicketViewingRepository + Send + Sync
{
    pub fn new(ticket_viewing_repo:Arc<T>) -> Self{
        Self { ticket_viewing_repo }
    }

    pub async fn view(&self) -> Result<Vec<TicketEntity>>{
        let results = self.ticket_viewing_repo.view().await?;

        let mut quests_model = Vec::new();

        for result in results.into_iter() {
            quests_model.push(result);
        }

        Ok(quests_model)
    }
}