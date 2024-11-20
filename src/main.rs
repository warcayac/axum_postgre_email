mod routes;
mod models;
mod pages;
mod middleware;
mod config;
mod errors;
mod db;
mod utils;
mod mail;

use std::sync::Arc;
use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Method};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{self, CorsLayer};
use tracing_subscriber::filter::LevelFilter;

use db::db_client::DBClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: config::Config,
    pub db_client: DBClient,
}

// server : cargo watch -q -c -w src/ -x run
// tests  : cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init()
    ;
    let config = config::Config::init();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    ;
    let pool = match pool {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        },
        Err(e) => {
            println!("🔥 Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        },
    };
    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        // .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT])
    ;
    let app_state = AppState {
        env: config,
        db_client: DBClient::new(pool),
    };

    match tokio::net::TcpListener::bind("127.0.0.1:8080").await {
        Ok(listener) => {
            println!("\n🚀 Listening on {:?}", listener.local_addr().unwrap());
            let app = routes::app(Arc::new(app_state)).layer(cors);
            axum::serve(listener, app)
                .await
                .unwrap_or_else(|e| println!("\n->> Error serving application:\n    {e}"))
            ;
        },
        Err(e) => println!("\n->> Unable to connect to the server:\n    {e}"),
    }
}
