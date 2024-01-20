use crate::api::hello_world::handlers::handler;
use crate::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn get_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/hello", get(handler))
        .with_state(app_state)
}
