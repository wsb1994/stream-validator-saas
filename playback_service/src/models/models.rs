use dotenv::dotenv;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::env;
use tokio;

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
