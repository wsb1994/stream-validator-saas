use dotenv::dotenv;
use postgrest::Postgrest;
use serde::Deserialize;
use std::env;
use tokio;

use chrono::{Date, DateTime, Utc};

mod client;
mod models;
use models::models::Stream;

use client::rabbit_client::build_and_run_rabbitmq_system;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file
    loop {
        let build_and_run_rabbitmq_system = build_and_run_rabbitmq_system().await;
    }

    Ok(())
}
