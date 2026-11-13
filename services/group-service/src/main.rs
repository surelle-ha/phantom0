use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn service_info() -> Json<Value> {
    Json(json!({
        "service": "group-service",
        "status": "ok",
        "message": "Group management service is running",
        "responsibility": "Manages groups, memberships, and group-level access"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({ "service": "group-service", "status": "healthy" }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(service_info))
        .route("/health", get(health));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8333".to_owned());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
