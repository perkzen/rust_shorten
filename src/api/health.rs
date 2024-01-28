use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Health {
    message: String,
    timestamp: String,
}

#[utoipa::path(get, path = "/", tag = "Health",
responses(
(status = 200, description = "Health check", body = Health),
))]
pub async fn health_check() -> (StatusCode, Json<Health>) {
    let health = Health {
        message: String::from("Server is running! 🦀"),
        timestamp: Utc::now().to_string(),
    };

    (StatusCode::OK, Json(health))
}
