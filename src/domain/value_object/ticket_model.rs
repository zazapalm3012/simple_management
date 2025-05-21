use serde::{Deserialize, Serialize};
use crate::domain::entities::tickets::AddTicketEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTicketModel{
    pub user_id : i32,
    pub subject : String,
    pub issue_type : String,
    pub description : Option<String>,
    pub status : String,
    pub priority : String,
    pub reject_message: Option<String>,
    pub ticket_file : Option<String>,
}

impl AddTicketModel{
    pub fn to_entity(&self) -> AddTicketEntity{
        AddTicketEntity{
            user_id : self.user_id,
            subject : self.subject.clone(),
            issue_type : self.issue_type.clone(),
            description : self.description.clone(),
            status : crate::domain::value_object::ticket_statuses::TicketStatus::NotAssigned.to_string(),
            priority : crate::domain::value_object::ticket_priority::TicketPriority::NotAssigned.to_string(),
            reject_message: self.reject_message.clone(),
            ticket_file : self.ticket_file.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}