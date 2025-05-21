use serde::{Deserialize, Serialize};
use crate::domain::entities::users::RegisterUserEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserModel{
    pub username : String,
    pub password : String,
    pub email : String,
    pub user_address : Option<String>,
    pub organization_name : Option<String>,
    pub user_img : Option<String>,
    pub phone_number : Option<String>,
}

impl RegisterUserModel{
    pub fn to_entity(&self) -> RegisterUserEntity{
        RegisterUserEntity{
            username: self.username.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            user_address : self.user_address.clone(),
            organization_name : self.organization_name.clone(),
            user_img : self.user_img.clone(),
            phone_number : self.phone_number.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}