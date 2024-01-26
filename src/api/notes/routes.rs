use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::api::notes::handlers::{create_note, get_note};
use crate::AppState;

pub fn get_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/notes/:id", get(get_note))
        .route("/notes", post(create_note))
        .with_state(app_state)
}