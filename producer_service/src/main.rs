use dotenv::dotenv;
use tokio;

mod client;
mod models;


use client::rabbit_client::build_and_run_rabbitmq_system;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file
    
    loop {

        let _build_and_run_rabbitmq_system = build_and_run_rabbitmq_system().await;
    }

   // Ok(())
}
