use serde::Serialize;
use tokio;
use tokio::net::TcpListener;
use axum::{ extract::Json, routing::{get, put}, Router };
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //Define cors layer
    let cors = CorsLayer::new()
    .allow_origin(Any) // Allow any origin
    .allow_methods(Any) // Allow any method
    .allow_headers(Any); // Allow any header
    let app = Router::new()
    .route("/api/test", get(test)) // Test api response
    .layer(cors);

    let addr:SocketAddr = "127.0.0.1:6969".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener,app).await.unwrap();
}


// Test api response
async fn test() -> impl axum::response::IntoResponse {
    let message = "Hello World!".to_string();

    #[derive(Serialize)]
    struct Response {
        message: String
    }
    Json(Response {
        message
    })
}
