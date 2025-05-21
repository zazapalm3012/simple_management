use std::sync::Arc;

use tracing::{error, info};

use simple_management::config::config_loader;
use simple_management::infrastructure::postgres::postgres_connection;
use simple_management::infrastructure::axum_http::server::start;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let env_envy = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load .env file: {}", e);
            std::process::exit(1);
        },
    };
    info!("server has been loaded: {:#?}", env_envy.server);
    info!("database_url has been loaded: {:#?}", env_envy.database);
    
    let establish_connection = match postgres_connection::establish_connection(&env_envy.database.url) {
        Ok(database) => database,
        Err(e) => {
            error!("Failed to load .env file: {}", e);
            std::process::exit(1);
        },
    };
    info!("database establish: {:#?}", establish_connection);

    start(Arc::new(env_envy), Arc::new(establish_connection)).await.expect("Cannot start server");
   
} 

