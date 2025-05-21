use axum::{
    extract::State, http::{StatusCode}, middleware, routing::{get}, Extension, Json, Router
};
use std::sync::Arc;
use axum::response::IntoResponse;
use crate::{application::usecases::ticket_viewing::TicketViewingUseCase, domain::repositories::ticket_viewing::TicketViewingRepository, infrastructure::{axum_http::middlewares::users_authorization, postgres::{postgres_connection::PgPoolSquad, repositories::ticket_viewing::TicketViewingPostgres}}};



pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {

    let ticket_viewing_repository = TicketViewingPostgres::new(Arc::clone(&db_pool));
    let ticket_viewing_usecase = TicketViewingUseCase::new(Arc::new(ticket_viewing_repository));

    Router::new()
    .route("/ticket", get(view))
    .route_layer(middleware::from_fn(users_authorization))
    .with_state(Arc::new(ticket_viewing_usecase))
}

pub async fn view<T>(
    State(ticket_viewing_usecase): State<Arc<TicketViewingUseCase<T>>>,
    Extension(_user_id): Extension<i32>,
) -> impl IntoResponse
where
    T: TicketViewingRepository + Send + Sync,
{
    match ticket_viewing_usecase.view().await {
        Ok(ticket_model) => (StatusCode::OK, Json(ticket_model)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}