use serde::{Deserialize, Serialize};
use crate::domain::entities::tickets::{AddTicketEntity, EditTicketEntity};

use super::{ticket_priority::TicketPriority, ticket_statuses::TicketStatus};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditTicketModel {
    pub status: Option<TicketStatus>,
    pub priority: Option<TicketPriority>,
    pub reject_message: Option<String>,
}

impl EditTicketModel {
    pub fn to_entity(&self) -> EditTicketEntity {
        EditTicketEntity {
            status: self.status.clone().map(|s| s.to_string()),
            priority: self.priority.clone().map(|p| p.to_string()),
            reject_message: self.reject_message.clone(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}