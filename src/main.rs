use std::{sync::{Arc, Mutex}};

use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use rusqlite::Connection;
use serde_json::json;

mod ssh;

/*
#[derive(Debug)]
enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND, "Data not found".to_string()
            ),
            ApiError::InvalidInput(msg) => (
                StatusCode::BAD_REQUEST, msg
            ),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()
            )
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
*/



#[tokio::main]
async fn main() {
    ssh::login()
    .await;
    
    
    /* 

    let conn = Connection::open("./main.db").expect("failed to connect to the database");
    
    let db = Arc::new(Mutex::new(conn));

    let router = Router::new()
    .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, router)
        .await
        .expect("failed to start server");
*/
}