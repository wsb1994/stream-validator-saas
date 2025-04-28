
use dotenv::dotenv;
use lapin::BasicProperties;
use lapin::{
    Channel, Connection, ConnectionProperties,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
};
use models::models::Stream;
use std::env;
use tokio::time::{Duration, sleep};

use crate::models;
pub struct RabbitMQClient {
    pub connection: Connection,
    pub channel: Channel,
}

impl RabbitMQClient {
    pub async fn new() -> Result<Self, lapin::Error> {
        dotenv().ok(); // Load .env file

        // Get RabbitMQ connection URL from environment variables
        let rabbitmq_url = env::var("AMQP_ADDR")
            .unwrap_or_else(|_| "amqp://admin:admin@localhost:5672".to_string());

        // Open a connection to RabbitMQ server
        let connection =
            Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;

        // Open a channel
        let channel = connection.create_channel().await?;

        Ok(Self {
            connection,
            channel,
        })
    }

    pub async fn declare_queue(&self) -> Result<String, lapin::Error> {
        let queue_name = env::var("AMQP_QUEUE")
            .unwrap_or_else(|_| "video_playback_job".to_string());
        let queue = self
            .channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(queue.name().to_string())
    }

    pub async fn publish_message(
        &self,
        exchange_name: &str,
        routing_key: &str,
        message: Vec<u8>,
    ) -> Result<(), lapin::Error> {
        let args = BasicPublishOptions::default();
        self.channel
            .basic_publish(
                exchange_name,              // exchange name
                routing_key,                // routing key
                args,                       // publish options (default)
                &message,                   // message body as &[u8]
                BasicProperties::default(), // message properties (default)
            )
            .await?;

        Ok(())
    }

    pub async fn close(&self) -> Result<(), lapin::Error> {
        self.channel.close(200, "Bye").await?;
        self.connection.close(200, "Bye").await?;
        Ok(())
    }
}

pub async fn build_and_run_rabbitmq_system() {


    // Create the RabbitMQ client
    let client = match RabbitMQClient::new().await {
        Ok(client) => {client},
        Err(err) => {
            eprintln!("Failed to connect to RabbitMQ: {}", err);
            return;
        }
    };

    // Declare a queue
    let queue_name = match client.declare_queue().await {
        Ok(queue_name) => queue_name,
        Err(err) => {
            eprintln!("Failed to declare queue: {}", err);
            return;
        }
    };
    loop {
        let streams: Result<Vec<Stream>,_> = models::models::get_streams()
        .await;
        
        match streams{
            Ok(streams) => {
                sleep(Duration::from_millis(7000)).await;
        
                for stream in streams {
                    println!("ID: {:?}, Content: {:?}", stream.id, stream);
        
                    let message = serde_json::to_string(&stream).unwrap().as_bytes().to_vec();
        
                    let exchange_name = ""; // Use default exchange
                    let routing_key = &queue_name; // Use the queue name as the routing key
                    if let Err(err) = client
                        .publish_message(exchange_name, routing_key, message)
                        .await
                    {
                        eprintln!("Failed to publish message: {}", err);
                    }
                }
            },
            Err(_) =>{}
        }

       
    }
}
