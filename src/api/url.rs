use axum::{
    http::StatusCode,
    Json,
};
use axum::extract::State;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::AppState;


#[derive(Serialize, ToSchema)]
pub struct ShortUrl {
    short_url: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LongUrl {
    url: String,
}

#[utoipa::path(post, path = "/url", tag = "Url",
responses(
(status = 200, description = "Shorten URL", body = ShortUrl),
))]
pub async fn post_url(State(state): State<AppState>, payload: Json<LongUrl>) -> (StatusCode, Json<ShortUrl>) {
    let id = Uuid::new_v4().to_string();
    let short_id = id.split('-').next().unwrap();

    let _long_url: String = payload.url.clone();

    let mut redis = state.redis.lock().await;
    let _: () = redis.set(short_id, payload.url.clone()).await.unwrap();

    let short_url = ShortUrl {
        short_url: String::from(short_id),
    };

    (StatusCode::OK, Json(short_url))
}