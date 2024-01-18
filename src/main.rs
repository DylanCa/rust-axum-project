use axum::Router;
use hello_world;
use user;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(hello_world::routes::get_routes())
        .merge(users::routes::get_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
