use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello from Rust!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    println!("Server running on http://localhost:3000/api/hello");

    axum::serve(listener, app).await.unwrap();
}