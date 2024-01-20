use axum::Router;
use axum::routing::get;
use crate::api::hello_world::handlers::handler;

pub fn get_routes() -> Router {
    Router::new()
        .route("/hello", get(handler))
}
