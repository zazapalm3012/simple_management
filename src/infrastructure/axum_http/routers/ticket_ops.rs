use axum::{extract::{Path, State}, http::StatusCode, middleware, response::IntoResponse, routing::{ delete, patch, post, Router}, Extension, Json};

use std::sync::Arc;

use crate::{application::usecases::ticket_ops::TicketOpsUsecase, domain::{repositories::ticket_ops::TicketOpsRepository, value_object::ticket_model::{AddTicketModel, EditTicketModel}}, infrastructure::{axum_http::middlewares::users_authorization, postgres::{postgres_connection::PgPoolSquad, repositories::ticket_ops::TicketOpsPostgres}}};


pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {

    let ticket_ops_repository = TicketOpsPostgres::new(Arc::clone(&db_pool));
    let ticket_ops_usecase = TicketOpsUsecase::new(Arc::new(ticket_ops_repository));

    Router::new()
    .route("/", post(add))
    .route("/:ticket_id", delete(remove))
    .route("/edit/:ticket_id", patch(edit))
    .route_layer(middleware::from_fn(users_authorization))
    .with_state(Arc::new(ticket_ops_usecase))
}

pub async fn add<T>(
    State(ticket_ops_usecase): State<Arc<TicketOpsUsecase<T>>>,
    Extension(_user_id): Extension<i32>,
    Json(add_ticket_model): Json<AddTicketModel>,
) -> impl IntoResponse
where
    T: TicketOpsRepository + Send + Sync,
{
    match ticket_ops_usecase.add(add_ticket_model).await {
        Ok(ticket_id) => (StatusCode::CREATED, format!("Create successfully!, Ticket with id: {}", ticket_id)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn remove<T>(
    State(ticket_ops_usecase): State<Arc<TicketOpsUsecase<T>>>,
    Path(ticket_id): Path<i32>,
    Extension(_user_id): Extension<i32>,
) -> impl IntoResponse
where
    T: TicketOpsRepository + Send + Sync,
{
    match ticket_ops_usecase.remove(ticket_id).await {
        Ok(ticket_id) => (StatusCode::OK, format!("Ticket with ID {} deleted successfully!", ticket_id)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn edit<T>(
    State(ticket_ops_usecase): State<Arc<TicketOpsUsecase<T>>>,
    Extension(_user_id): Extension<i32>,
    Path(ticket_id): Path<i32>,
    Json(edit_ticket_model): Json<EditTicketModel>,
) -> impl IntoResponse
where
    T: TicketOpsRepository + Send + Sync,
{
    match ticket_ops_usecase.edit(ticket_id, edit_ticket_model).await {
        Ok(ticket_id) => (StatusCode::OK, format!("Ticket with ID {} updated successfully!", ticket_id)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}