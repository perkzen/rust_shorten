mod api;
mod app_state;

use axum::{routing::{get, post}, Router, http};
use crate::api::{
    health::health_check,
    url::{add_url, delete_url, redirect_to},
};
use dotenv::dotenv;
use std::env;
use std::io::Error;
use std::net::{SocketAddr};
use axum::http::Method;
use axum::routing::delete;
use tokio::net::TcpListener;
use utoipa::{OpenApi};
use utoipa_rapidoc::RapiDoc;
use tower_http::cors::{Any, CorsLayer};
use crate::app_state::AppState;


#[tokio::main]
async fn main() -> Result<(), Error> {
    #[derive(OpenApi)]
    #[openapi(
    paths(api::health::health_check, api::url::post_url, api::url::delete_url, api::url::redirect_to),
    components(schemas(api::health::Health, api::url::ShortUrl, api::url::LongUrl, api::generic_response::GenericResponse)),
    info(title = "Rust Shortener", description = "Rust URL Shortener")
    )]
    struct ApiDoc;

    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");

    let state = AppState::new(redis_url).await;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([http::header::AUTHORIZATION, http::header::ACCEPT]);


    let app = Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/docs"))
        .route("/", get(health_check))
        .route("/:id", get(redirect_to))
        .route("/url", post(add_url))
        .route("/url/:id", delete(delete_url))
        .layer(cors)
        .with_state(state);


    let address = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    let listener = TcpListener::bind(&address).await?;
    println!("Documentation available on http://{}/docs", address);
    axum::serve(listener, app.into_make_service()).await
}


