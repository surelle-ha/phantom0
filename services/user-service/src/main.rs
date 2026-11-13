use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn service_info() -> Json<Value> {
    Json(json!({
        "service": "user-service",
        "status": "ok",
        "message": "User management service is running",
        "responsibility": "Manages Phantom0 user profiles and preferences"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({ "service": "user-service", "status": "healthy" }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(service_info))
        .route("/health", get(health));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8331".to_owned());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
