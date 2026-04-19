use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    Router,
    response::{Sse, sse::{Event, KeepAlive}},
    routing::{get, post},
};
use futures::{Stream, stream};
use rusqlite::Connection;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

mod routes;
mod ssh;
mod structs;

#[tokio::main]
async fn main() {
    let conn = Connection::open("./main.db").expect("failed to connect to the database");
    
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS servers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            token_hash TEXT NOT NULL,
            last_heartbeat INTEGER,
            created_at INTEGER NOT NULL,
            
            -- stats (latest values, overwritten every heartbeat)
            cpu REAL,
            ram_used INTEGER,
            ram_total INTEGER,
            disk_used INTEGER,
            disk_total INTEGER,
            net_rx INTEGER,
            net_tx INTEGER
        ) STRICT",
        (),
    )
    .expect("failed to create servers table");

    conn.execute(
        "
        INSERT OR IGNORE INTO servers (id, name, token_hash, last_heartbeat, created_at)
        VALUES (
            'vps_demo123',
            'Demo Server',
            '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8',
            strftime('%s', 'now'),
            strftime('%s', 'now')
        )
        ",
        (),
    )
    .expect("failed to insert test server");

    let db = Arc::new(Mutex::new(conn));

    let router = Router::new()
        .route("/servers", get(routes::server::get_all))
        .route("/server", post(routes::server::connect_server))
        .route("/heartbeat", post(routes::server::heartbeat))
        .route("/stream", get(stream_handler))        
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, router)
        .await
        .expect("failed to start server");
}

async fn stream_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

