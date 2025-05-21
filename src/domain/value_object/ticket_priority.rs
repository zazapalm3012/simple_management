use serde::{Deserialize, Serialize};
use core::fmt;
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketPriority {
    #[default]
    NotAssigned,
    High,
    Medium,
    Low,
}

impl fmt::Display for TicketPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        let result = match self{
            TicketPriority::NotAssigned => "NotAssigned",
            TicketPriority::High => "High",
            TicketPriority::Medium => "Medium",
            TicketPriority::Low => "Low",
        };
        write!(f, "{}",result)
    }
}