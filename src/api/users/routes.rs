use crate::api::users::handlers::{create_user, get_user, login};
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn get_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/users/:uuid", get(get_user))
        .route("/users", post(create_user))
        .with_state(app_state)
}
