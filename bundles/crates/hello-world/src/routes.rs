use axum::Router;
use axum::routing::get;
use crate::handlers::handler;

pub fn get_routes() -> Router {
    Router::new()
        .route("/hello", get(handler))
}
