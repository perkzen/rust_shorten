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
use crate::api::responses::{Response};
use crate::AppState;


#[derive(Serialize, Deserialize, ToSchema, Debug)]
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
pub async fn add_url(State(state): State<AppState>, payload: Json<LongUrl>) -> (StatusCode, Json<ShortUrl>) {
    let id = Uuid::new_v4().to_string();
    let short_id = id.split('-').next().unwrap();

    let mut redis = state.redis.lock().await;
    let _: () = redis.set(&short_id, &payload.url).await.unwrap();

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
(status = 404, description = "URL not found", body = api::responses::Response),
))]
pub async fn delete_url(State(state): State<AppState>, Path(id): Path<String>) -> (StatusCode, Json<Response>) {
    let mut redis = state.redis.lock().await;
    let _: () = redis.del(&id).await.unwrap();

    (StatusCode::OK, Json(Response::success(format!("URL {} deleted", id))))
}

#[utoipa::path(get, path = "/{id}", tag = "Redirect",
params(
("id" = String, Path, description = "Short URL ID"),
),
responses(
(status = 302, description = "Redirect to URL"),
(status = 404, description = "URL not found", body = api::responses::Response),
))]
pub async fn redirect_to(State(state): State<AppState>, Path(id): Path<String>) -> Result<Redirect, (StatusCode, Json<Response>)> {
    let mut redis = state.redis.lock().await;
    let url: Result<String, _> = redis.get(&id).await;

    match url {
        Ok(url) => Ok(Redirect::temporary(&url)),
        Err(_) => Err((StatusCode::NOT_FOUND, Json(Response::error(format!("URL {} not found", id)))))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::{StatusCode},
        Router,
        routing::{get, post, delete},
    };
    use axum_test::TestServer;
    use serde_json::json;


    async fn create_server() -> TestServer {
        let state = AppState::new("redis://localhost:6379".to_string()).await;

        let app = Router::new()
            .route("/url", post(add_url))
            .route("/url/:id", delete(delete_url))
            .route("/:id", get(redirect_to))
            .with_state(state);

        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    async fn test_post_url() {
        let server = create_server().await;

        let res = server.post("/url").json(
            &json!({
                "url": "https://www.rust-lang.org/"
            })).await;

        let body: ShortUrl = res.json();

        assert_eq!(res.status_code(), StatusCode::OK);
        assert_eq!(body.short_url.len(), 8);
    }

    #[tokio::test]
    async fn test_delete_url() {
        let server = create_server().await;

        let res = server.post("/url").json(
            &json!({
                "url": "https://www.rust-lang.org/"
            })).await;

        let body: ShortUrl = res.json();

        let res = server.delete(&format!("/url/{}", body.short_url)).await;

        let status = res.status_code();
        assert_eq!(status, StatusCode::OK);

        let res_body: Response = res.json();
        assert_eq!(res_body.message, format!("URL {} deleted", body.short_url));
    }

    #[tokio::test]
    async fn test_redirect_to() {
        let server = create_server().await;

        let res = server.get("/test").await;

        assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
    }
}