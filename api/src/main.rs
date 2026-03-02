use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct NameInput {
    name: String,
}

#[derive(Serialize)]
struct HelloResponse {
    name: String,
    message: String,
    timestamp: String,
}

async fn root() -> &'static str {
    "Rust API is running. Try /health or /api/hello"
}

async fn health() -> &'static str {
    "OK"
}

async fn hello_get() -> Json<HelloResponse> {
    let timestamp = OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap();

    Json(HelloResponse {
        name: "friend".to_string(),
        message: "Hello from Rust!".to_string(),
        timestamp,
    })
}

async fn hello_post(Json(payload): Json<NameInput>) -> Json<HelloResponse> {
    let trimmed = payload.name.trim();

    let name = if trimmed.is_empty() {
        "friend".to_string()
    } else {
        trimmed.to_string()
    };

    let timestamp = OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap();

    Json(HelloResponse {
        name: name.clone(),
        message: format!("Hello, {name}!"),
        timestamp,
    })
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/hello", get(hello_get))
        .route("/api/hello", post(hello_post))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");

    println!("Server running on http://{addr}");

    axum::serve(listener, app).await.unwrap();
}