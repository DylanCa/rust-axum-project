use crate::api::{hello_world, notes, redirection, users};
use crate::config::connection::AppState;
use axum::Router;
use std::sync::Arc;

pub fn get_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(hello_world::routes::get_routes(app_state.clone()))
        .merge(users::routes::get_routes(app_state.clone()))
        .merge(notes::routes::get_routes(app_state.clone()))
        .merge(redirection::routes::get_routes(app_state.clone()))
}

pub fn get_login(app_state: Arc<AppState>) -> Router {
    users::routes::get_login(app_state)
}
