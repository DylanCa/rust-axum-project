use crate::api::auth_mw::auth_required;
use axum::body::Body;
use axum::middleware::from_fn;
use axum::response::Response;
use axum::{middleware, Router};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, MySqlPool, Pool};
use std::sync::Arc;
use serde_json::Value;
use socketioxide::extract::{AckSender, Bin, Data, SocketRef};
use socketioxide::SocketIo;
use tower_cookies::CookieManagerLayer;
use log::info;

mod api;
mod ctx;
mod errors;
pub use self::errors::Error;

#[derive(Debug)]
pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 Rust Playground Server 🌟");

    let pool = get_db_pool().await;
    let app_state = Arc::new(AppState { db: pool.clone() });

    let (socket_layer, io) = SocketIo::new_layer();

    io.ns("/ws", on_connect);

    let routes_api =
        api::routes::get_routes(app_state.clone()).route_layer(from_fn(auth_required::<Body>));


    let router = Router::new()
        .merge(api::routes::get_login(app_state.clone()))
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(socket_layer);

    let serv_addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&serv_addr).await.unwrap();
    println!("✅ Server started successfully at {serv_addr}");

    axum::serve(listener, router).await.unwrap();
}

async fn get_db_pool() -> Pool<MySql> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> main_response_mapper");
    println!();

    res
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("auth", data).ok();

    socket.on(
        "message",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
           socket.bin(bin).emit("message-back", data).ok();
        }
    );

    socket.on(
        "message-with-ack",
        |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            ack.bin(bin).send(data).ok();
        }
    );
}