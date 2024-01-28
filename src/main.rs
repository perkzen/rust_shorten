use axum::{
    routing::{get},
    Router,
    http::StatusCode,
    Json,
};
use chrono::{Utc};
use serde::Serialize;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let app = Router::new()
        .route("/", get(health_check));

    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running on port http://{}", addr);
    axum::serve(listener, app).await.unwrap()
}


#[derive(Serialize)]
struct Health {
    message: String,
    timestamp: String,
}

async fn health_check() -> (StatusCode, Json<Health>) {
    let health = Health {
        message: String::from("Server is running! 🦀"),
        timestamp: Utc::now().to_string(),
    };

    (StatusCode::OK, Json(health))
}