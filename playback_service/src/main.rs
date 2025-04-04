use dotenv::dotenv;
use std::env;

use models::models::Stream;

use futures::StreamExt;
use lapin::{
    Connection, ConnectionProperties, Error as LapinError, Result as LapinResult, options::*,
    protocol::queue, types::FieldTable,
}; //Import lapin::Error
use serde_json;
use video::video::check_hls_stream;

pub mod models;
pub mod video;


#[tokio::main]
async fn main() -> LapinResult<()> {
    dotenv().ok();

    let addr =
        std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://admin:admin@localhost:5672".into());
    let queue_name = std::env::var("AMQP_QUEUE").unwrap_or_else(|_| "playback_service".into());
    println!("addr: {}", addr);
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;


    for i in 0..16 {
        let mut consumer = channel
            .basic_consume(
                &queue_name,
                &format!("playback_service{}", i),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        tokio::spawn(async move {
            loop {
                while let Some(delivery) = consumer.next().await {
                    let delivery = match delivery {
                        Ok(d) => d,
                        Err(err) => {
                            eprintln!("Error in consumer: {:?}", err);
                            continue;
                        }
                    };

                    let bytes = &delivery.data;
                    let message: Result<Stream, serde_json::Error> = serde_json::from_slice(bytes);

                    match message {
                        Ok(result) => {
                            match check_hls_stream(&result.url) {
                                Ok(result_bool) => {
                                    if result_bool {
                                        println!("{} had video playback", result.name);
                                    } else {
                                        println!("{} had no video playback", result.name);
                                    }
                                    
                                }
                                Err(_value) => {
                                    eprintln!("Stream check failed for {}", result.name);
                                    continue; // Skip acknowledgment if an error occurs
                                }
                            }

                            // Acknowledge only after successful processing
                            if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
                                eprintln!("Failed to acknowledge message: {:?}", err);
                            }
                        }
                        Err(err) => {
                            eprintln!("Deserialization error: {:?}", err);
                        }
                    }
                }
            }
        });
    }
    loop {}
}
