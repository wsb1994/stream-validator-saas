use serde::{Deserialize, Serialize};
use std::env;
use tokio;

use crate::client::db_client::get_client;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug, Serialize)]
pub struct Stream {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub url: String,
    pub is_enabled: bool,
    pub is_playing: bool,
    pub last_playback: Option<DateTime<Utc>>,
    pub tag: String,
}

pub async fn get_streams() -> Result<Vec<Stream>, Box<dyn std::error::Error>> {
    let client = get_client();
    let client_guard = client.lock().unwrap();

    // Assuming you need to query the "streams" table.
    let response = client_guard
        .db_client
        .from("streams") // Table name
        .select("*") // Select all columns
        .execute()
        .await?
        .text()
        .await?;

    // Parse the response text into a Vec<Stream> (assuming the response is JSON)
    let streams: Vec<Stream> = serde_json::from_str(&response)?;

    Ok(streams)
}
