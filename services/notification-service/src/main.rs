use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn service_info() -> Json<Value> {
    Json(json!({
        "service": "notification-service",
        "status": "ok",
        "message": "Notification service is running",
        "responsibility": "Delivers security alerts and platform notifications"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({ "service": "notification-service", "status": "healthy" }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(service_info))
        .route("/health", get(health));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8334".to_owned());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
