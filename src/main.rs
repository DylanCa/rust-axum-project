use axum::response::Response;
use axum::{middleware, Router};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, MySqlPool, Pool};
use std::sync::Arc;
use axum::body::Body;
use axum::middleware::from_fn;
use axum::routing::post;
use tower_cookies::CookieManagerLayer;
use crate::api::auth_mw::auth_required;
use crate::api::users::handlers::login;

mod api;
mod errors;
mod ctx;
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

    let routes_api = api::routes::get_routes(app_state.clone())
        .route_layer(from_fn(auth_required::<Body>));

    let router = Router::new()
        .merge(api::routes::get_login(app_state.clone()))
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());

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
