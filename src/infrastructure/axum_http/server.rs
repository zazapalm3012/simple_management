use std::{net::SocketAddr, sync::Arc};

use axum::{http::Method, Router};
use anyhow::{Ok, Result};
use tower_http::{cors::{Any, CorsLayer}, limit::RequestBodyLimitLayer, trace::TraceLayer};
use crate::{config::config_model::DotEnvyConfig, infrastructure::{postgres::postgres_connection::PgPoolSquad}};
use crate::infrastructure::axum_http::routers;

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
    .nest("/authentication", routers::authentication::routes(Arc::clone(&db_pool)))
    .nest("/user", routers::users::routes(Arc::clone(&db_pool)))
    .nest("/ticket", routers::ticket_ops::routes(Arc::clone(&db_pool)))
    .nest("/view", routers::ticket_viewing::routes(Arc::clone(&db_pool)))
    .layer(RequestBodyLimitLayer::new(
        (config.server.body_limit * 1024 * 1024).try_into()?,
    ))
    .layer(
        CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
            ])
            .allow_origin(Any),
    )
    .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    // run our app with hyper, listening globally on port config
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}