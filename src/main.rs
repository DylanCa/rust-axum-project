use axum::Router;
mod api;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", api::routes::get_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
