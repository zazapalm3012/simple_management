use crate::infrastructure::jwt_authentication::authentication_model::LoginModel;
use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::{config_loader::get_stage, stage::Stage},
    domain::repositories::users::UserRepository,
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::users::UsersPostgres,
    },
};
use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let users_repository = UsersPostgres::new(db_pool);
    let authentication_usecase = AuthenticationUseCase::new(Arc::new(users_repository));

    Router::new()
        .route("/login", post(users_login))
        .route("/refresh-token", post(users_refresh_token))
        .with_state(Arc::new(authentication_usecase))
}

pub async fn users_login<T>(
    State(authentication_usecase): State<Arc<AuthenticationUseCase<T>>>,
    Json(login_user_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync,
{
    match authentication_usecase.users_login(login_user_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login success").into_response()
        }
        Err(err) => (StatusCode::UNAUTHORIZED, err.to_string()).into_response(),
    }
}

    pub async fn users_refresh_token<T>(
        State(authentication_usecase): State<Arc<AuthenticationUseCase<T>>>,
        jar: CookieJar,
    ) -> impl IntoResponse
    where
        T: UserRepository + Send + Sync,
    {
        if let Some(rft) = jar.get("rft") {
            let refesh_token = rft.value().to_string();

            let response = match authentication_usecase
                .users_refresh_token(refesh_token)
                .await
            {
                Ok(passport) => {
                    let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                        .path("/")
                        .same_site(cookie::SameSite::Lax)
                        .http_only(true)
                        .max_age(Duration::days(14));

                    let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                        .path("/")
                        .same_site(cookie::SameSite::Lax)
                        .http_only(true)
                        .max_age(Duration::days(14));

                    if get_stage() == Stage::Production {
                        act_cookie = act_cookie.secure(true);
                        rft_cookie = rft_cookie.secure(true);
                    }

                    let mut headers = HeaderMap::new();

                    headers.append(
                        header::SET_COOKIE,
                        HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                    );

                    headers.append(
                        header::SET_COOKIE,
                        HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                    );

                    (StatusCode::OK, headers, "Login success").into_response()
                }
                Err(err) => (StatusCode::UNAUTHORIZED, err.to_string()).into_response(),
            };
            return response;
        }
    (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
}
