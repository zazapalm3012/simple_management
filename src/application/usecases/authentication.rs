use crate::{config::config_loader::get_users_secret, domain::repositories::users::UserRepository, infrastructure::{argon2_hashing, jwt_authentication::{self, authentication_model::LoginModel, jwt_model::{Claims, Passport, Roles}}}};
use std::sync::Arc;
use anyhow::{Ok, Result};

pub struct AuthenticationUseCase<T>
where
    T: UserRepository + Send + Sync
{
    users_repository: Arc<T>,
}

impl<T> AuthenticationUseCase<T>
where
    T: UserRepository + Send + Sync,
{
    pub fn new(users_repository: Arc<T>) -> Self{
        Self { users_repository }
    }

    pub async fn users_login(&self, login_user_model: LoginModel) -> Result<Passport>{
        let secret_env = get_users_secret()?;
        let user = self.users_repository.find_by_username(login_user_model.username.clone()).await?;

        let original_password = user.password;
        let login_password = login_user_model.password;

        if !argon2_hashing::verify(login_password, original_password)?  {
            return Err(anyhow::anyhow!("Invalid password"))
        }
        
        let access_token_claims = Claims{
            sub:  user.id.to_string(),
            role: Roles::User,
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };
        let refresh_token_claims = Claims{
            sub:  user.id.to_string(),
            role: Roles::User,
            exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };
        let access_token = jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token = jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport{
            access_token,
            refresh_token
        })
    }
    pub async fn users_refresh_token(&self, refresh_token: String) -> Result<Passport>{
        let secret_env = get_users_secret()?;

        let claims = jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;
        
        let access_token_claims = Claims{
            sub:  claims.sub.clone(),
            role: Roles::User,
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };
        let refresh_token_claims = Claims{
            sub:  claims.sub,
            role: Roles::User,
            exp: claims.exp,
            iat: chrono::Utc::now().timestamp() as usize,
        };
        let access_token = jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token = jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport{
            access_token,
            refresh_token
        })
    }
}