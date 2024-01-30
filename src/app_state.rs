use std::sync::Arc;
use redis::aio::Connection;
use redis::Client;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub redis: Arc<Mutex<Connection>>,
}

impl AppState {
    pub async fn new(redis_url: String) -> Self {
        let redis_client = Client::open(redis_url).expect("Failed to connect to Redis");

        let connection = redis_client.get_async_connection().await
            .expect("Failed to establish an async connection to Redis.");

        Self {
            redis: Arc::new(Mutex::new(connection)),
        }
    }
}

