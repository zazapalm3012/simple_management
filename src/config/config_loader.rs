use anyhow::{Ok, Result};

use super::{config_model::{Database, DotEnvyConfig, Server, UsersSecret}, stage::Stage};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server{
        port: std::env::var("SERVER_PORT").expect("SERVER_PORT is invalid").parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT").expect("SERVER_BODY_LIMIT is invalid").parse()?,
        timeout: std::env::var("SERVER_TIMEOUT").expect("SERVER_TIMEOUT is invalid").parse()?,
    };
    let database = Database{
        url: std::env::var("DATABASE_URL").expect("Not found database url"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_users_secret() -> Result<UsersSecret> {
    dotenvy::dotenv().ok();

    Ok(UsersSecret {
        secret: std::env::var("JWT_USER_SECRET").expect("JWT_USER_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_USER_REFRESH_SECRET").expect("JWT_USER_REFRESH_SECRET is invalid"),
    })
}

pub fn get_stage() -> Stage{
    dotenvy::dotenv().ok();

    let stage = std::env::var("STAGE").expect("STAGE is invalid");
    Stage::try_from(&stage).unwrap_or_default()
}