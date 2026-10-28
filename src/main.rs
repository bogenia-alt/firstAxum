mod hello;

use axum::{Router, routing::get};
use log::info;

const ADDR_SERVER: &str = "0.0.0.0:7890";

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let app = Router::new()
        .route("/hello", get(hello::hello_handler))
        .route("/health", get(health_handler));
    let listener = tokio::net::TcpListener::bind(ADDR_SERVER).await.unwrap();
    info!("Axum is started on {}", ADDR_SERVER);

    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> &'static str {
    info!("[health_handler] get query");
    "OK!\n"
}
