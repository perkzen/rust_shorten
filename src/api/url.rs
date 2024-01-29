use axum::{
    http::StatusCode,
    Json,
};
use axum::extract::{Path, State};
use axum::response::Redirect;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};
use uuid::Uuid;
use crate::api::generic_response::GenericResponse;
use crate::AppState;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct ShortUrl {
    short_url: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
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

    let mut redis = state.redis.lock().await;
    let _: () = redis.set(short_id, payload.url.clone()).await.unwrap();

    let short_url = ShortUrl {
        short_url: String::from(short_id),
    };

    (StatusCode::OK, Json(short_url))
}


#[utoipa::path(delete, path = "/url/{id}", tag = "Url",
params(
("id" = String, Path, description = "Short URL ID"),
),
responses(
(status = 200, description = "Delete URL"),
(status = 404, description = "URL not found"),
))]
pub async fn delete_url(State(state): State<AppState>, Path(id): Path<String>) -> (StatusCode, Json<GenericResponse>) {
    let mut redis = state.redis.lock().await;
    let _: () = redis.del(&id).await.unwrap();

    (StatusCode::OK, Json(GenericResponse {
        message: format!("URL {} deleted", id),
    }))
}

#[utoipa::path(get, path = "/{id}", tag = "Redirect",
params(
("id" = String, Path, description = "Short URL ID"),
),
responses(
(status = 404, description = "URL not found"),
))]
pub async fn redirect_to(State(state): State<AppState>, Path(id): Path<String>) -> Result<Redirect, (StatusCode, Json<GenericResponse>)> {
    let mut redis = state.redis.lock().await;
    let url: Result<String, _> = redis.get(&id).await;

    match url {
        Ok(url) => Ok(Redirect::temporary(&url)),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(GenericResponse {
            message: format!("URL {} not found", id),
        })))
    }
}