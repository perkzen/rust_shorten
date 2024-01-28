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
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;


#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
    paths(health_check),
    components(schemas(Health)),
    tags((name = "Rust Shortener", description = "Rust URL Shortener")
    ))]
    struct ApiDoc;

    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let app = Router::new()
        .merge(SwaggerUi::new("/swag").url("/api-docs.json", ApiDoc::openapi()))
        .route("/", get(health_check));

    let addr = format!("127.0.0.1:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running on port http://{}/swag", addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}


#[derive(Serialize, ToSchema)]
struct Health {
    message: String,
    timestamp: String,
}

#[utoipa::path(
get,
path = "/",
responses(
(status = 200, description = "Health check", body = Health),
))]
async fn health_check() -> (StatusCode, Json<Health>) {
    let health = Health {
        message: String::from("Server is running! 🦀"),
        timestamp: Utc::now().to_string(),
    };

    (StatusCode::OK, Json(health))
}