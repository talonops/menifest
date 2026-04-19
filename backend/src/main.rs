use std::sync::Arc;

use axum::{Router, routing::post};
use rusqlite::Connection;

use tokio::sync::Mutex;

mod routes;
mod ssh;
mod structs;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./main.db").expect("failed to connect to the database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS servers (
            id  TEXT PRIMARY KEY,  
            name TEXT NOT NULL,
            token_hash TEXT NOT NULL,
            last_heartbeat INTEGER,
            created_at INTEGER NOT NULL
         )",
        (),
    )
    .expect("failed to create servers table");

    let db = Arc::new(Mutex::new(conn));

    let router = Router::new()
        .route("/server", post(routes::server::connect_server))
        .route("/agent/heartbeat", post(routes::server::heartbeat))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, router)
        .await
        .expect("failed to start server");
}
