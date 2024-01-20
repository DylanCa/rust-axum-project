use axum::Router;
use crate::api::{hello_world, users};

pub fn get_routes() -> Router {
    Router::new()
        .merge(hello_world::routes::get_routes())
        .merge(users::routes::get_routes())
}
