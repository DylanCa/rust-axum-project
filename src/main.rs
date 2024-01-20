use axum::Router;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, MySqlPool, Pool};
use std::sync::Arc;

mod api;

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

    let router = Router::new().nest("/api", api::routes::get_routes(app_state.clone()));

    let serv_addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&serv_addr).await.unwrap();
    println!("✅ Server started successfully at {serv_addr}");

    axum::serve(listener, router).await.unwrap();
}

async fn get_db_pool() -> Pool<MySql> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match MySqlPoolOptions::new()
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
    };

    pool
}
