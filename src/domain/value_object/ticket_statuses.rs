use serde::{Deserialize, Serialize};
use core::fmt;
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketStatus {
    #[default]
    NotAssigned,
    Rejected,
    Open,
    Closed
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        let result = match self{
            TicketStatus::NotAssigned => "NotAssigned",
            TicketStatus::Rejected => "Rejected",
            TicketStatus::Open => "Open",
            TicketStatus::Closed => "Closed",
        };
        write!(f, "{}",result)
    }
}