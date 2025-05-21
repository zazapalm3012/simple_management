use diesel::prelude::*;
use crate::infrastructure::postgres::schema::tickets;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable,Deserialize,Serialize, Selectable, Queryable)]
#[diesel(table_name = tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TicketEntity {
    pub id : i32,
    pub user_id : i32,
    pub subject : String,
    pub issue_type : String,
    pub description : Option<String>,
    pub ticket_file : Option<String>,
    pub reject_message: Option<String>,
    pub status : Option<String>,
    pub priority : Option<String>,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = tickets)]
pub struct AddTicketEntity {
    pub user_id : i32,
    pub subject : String,
    pub issue_type : String,
    pub description : Option<String>,
    pub status : String,
    pub priority : String,
    pub reject_message: Option<String>,
    pub ticket_file : Option<String>,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
}