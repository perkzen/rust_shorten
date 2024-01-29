mod api;

use axum::{
    routing::{get},
    Router,
};
use dotenv::dotenv;
use std::env;
use std::io::Error;
use std::net::{SocketAddr};
use tokio::net::TcpListener;
use utoipa::{OpenApi};
use utoipa_rapidoc::RapiDoc;
use crate::api::health::health_check;

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[derive(OpenApi)]
    #[openapi(
    paths(api::health::health_check),
    components(schemas(api::health::Health)),
    info(title = "Rust Shortener", description = "Rust URL Shortener")
    )]
    struct ApiDoc;

    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let app = Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/docs"))
        .route("/", get(health_check));

    let address = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    let listener = TcpListener::bind(&address).await?;
    println!("Documentation available on http://{}/docs", address);
    axum::serve(listener, app.into_make_service()).await
}


