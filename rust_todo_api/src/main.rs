mod routes;
mod models;
mod state;

use axum::{Router};
use routes::{health::health_check, todos::{list_todos, create_todo}};
use state::AppState;
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();

    let app = Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/todos", axum::routing::get(list_todos).post(create_todo))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✅ Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

