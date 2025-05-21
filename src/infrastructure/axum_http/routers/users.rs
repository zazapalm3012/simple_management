use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{ post, Router}, Json};
use crate::{application::usecases::users::UserUseCase, domain::{repositories::users::UserRepository, value_object::user_model::RegisterUserModel}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::users::UsersPostgres}};




pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router{
    let users_repository = UsersPostgres::new(db_pool);
    let users_usecase = UserUseCase::new(Arc::new(users_repository));

    Router::new()
    .route("/", post(register))
    .with_state(Arc::new(users_usecase))
}

pub async fn register<T>(
    State(users_usecase): State<Arc<UserUseCase<T>>>,
    Json(register_user_model): Json<RegisterUserModel>,
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync,
{
    match users_usecase.register(register_user_model).await {
        Ok(user_id) => (StatusCode::CREATED, format!("Register user with id: {}", user_id)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}