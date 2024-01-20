use axum::Router;
use axum::routing::{get, post};
use crate::users::handlers::{create_user, get_user};

pub fn get_routes() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_user))
}
