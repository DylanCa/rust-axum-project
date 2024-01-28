use crate::config::connection::get_router;
use dotenv::dotenv;
use log::info;

mod api;
mod config;
mod ctx;
mod errors;

pub use self::errors::Error;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("🌟 Rust Playground Server 🌟");

    let serv_addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&serv_addr).await.unwrap();
    info!("✅ Server started successfully at {serv_addr}");

    let router = get_router().await;
    axum::serve(listener, router).await.unwrap();
}
