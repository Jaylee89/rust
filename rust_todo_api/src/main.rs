mod routes;
mod models;
mod state;

use axum::{Router, routing::get};
use routes::{health::health_check, todos::{list_todos, create_todo}};
use state::AppState;
use std::net::SocketAddr;
use tracing_subscriber;
use tower_http::limit::RequestBodyLimitLayer;

const MAX_REQUEST_SIZE: usize = 1024 * 10; // 10KB

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/todos", get(list_todos).post(create_todo))
        .layer(RequestBodyLimitLayer::new(MAX_REQUEST_SIZE))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Listening on http://{}", addr);
    
    // Configure TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    // Serve with hyper
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await
        .unwrap();
}
