mod api;

use axum::{routing::{get, post}, Router};
use crate::api::{
    health::health_check,
    url::{post_url, delete_url},
};
use dotenv::dotenv;
use std::env;
use std::io::Error;
use std::net::{SocketAddr};
use std::sync::{Arc};
use axum::routing::delete;
use redis::aio::Connection;
use redis::Client;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use utoipa::{OpenApi};
use utoipa_rapidoc::RapiDoc;


#[derive(Clone)]
pub struct AppState {
    redis: Arc<Mutex<Connection>>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[derive(OpenApi)]
    #[openapi(
    paths(api::health::health_check, api::url::post_url, api::url::delete_url),
    components(schemas(api::health::Health, api::url::ShortUrl, api::url::LongUrl)),
    info(title = "Rust Shortener", description = "Rust URL Shortener")
    )]
    struct ApiDoc;

    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");

    let redis_client = Client::open(redis_url).expect("Failed to connect to Redis");

    let state = AppState {
        redis: Arc::new(Mutex::new(redis_client.get_async_connection().await.unwrap())),
    };

    let app = Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/docs"))
        .route("/", get(health_check))
        .route("/url", post(post_url))
        .route("/url/:id", delete(delete_url))
        .with_state(state);


    let address = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    let listener = TcpListener::bind(&address).await?;
    println!("Documentation available on http://{}/docs", address);
    axum::serve(listener, app.into_make_service()).await
}


