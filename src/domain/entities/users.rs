use diesel::prelude::*;
use crate::infrastructure::postgres::schema::users;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id : i32,
    pub username : String,
    pub password : String,
    pub email : String,
    pub user_address : Option<String>,
    pub organization_name : Option<String>,
    pub user_img : Option<String>,
    pub phone_number : Option<String>,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct RegisterUserEntity{
    pub username : String,
    pub password : String,
    pub email : String,
    pub user_address : Option<String>,
    pub organization_name : Option<String>,
    pub user_img : Option<String>,
    pub phone_number : Option<String>,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
}