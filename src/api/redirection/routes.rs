use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::api::redirection::handlers::{create_redirection, get_redirection_url};
use crate::AppState;

pub fn get_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/redirect", get(get_redirection_url))
        .route("/redirect/url", post(create_redirection))
        .with_state(app_state)
}
