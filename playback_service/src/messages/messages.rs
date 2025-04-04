use async_nats::Client;

use std::sync::OnceLock;
use std::sync::Arc;
use std::env;
use tokio::sync::{futures, Mutex};
use dotenv::dotenv;

pub struct NatsClient {
    pub nats_client: Client,
}

impl NatsClient {
    /// Asynchronously create a new NATS client.
    pub async fn new() -> Self {
        dotenv().ok(); // Load .env file
        // Use a NATS-specific address (change the env var name as needed)
        let addr = env::var("NATS_ADDR").unwrap_or_else(|_| "nats://localhost:4222".to_string());
        println!("{}", addr);
        // Connect asynchronously to the NATS server
        let nats_client = async_nats::connect(addr)
            .await
            .expect("Failed to connect to NATS");
        Self { nats_client }
    }
}

// Global instance of our NATS client wrapped in Arc<Mutex<...>>
static GLOBAL_CLIENT: OnceLock<Arc<Mutex<NatsClient>>> = OnceLock::new();

/// Asynchronously obtain the global NATS client.
/// The first call will initialize the client.
pub async fn get_client() -> Arc<Mutex<NatsClient>> {
    // If already initialized, clone and return it.
    if let Some(client) = GLOBAL_CLIENT.get() {
        return client.clone();
    }
    // Otherwise, create a new NatsClient
    let nats_client = NatsClient::new().await;
    let arc_client = Arc::new(Mutex::new(nats_client));
    // Set the global instance; ignore the error if it was set in the meantime.
    let _ = GLOBAL_CLIENT.set(arc_client.clone());
    arc_client
}
