use core::sync;
use dotenv::dotenv;
use postgrest::Postgrest;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};

pub struct Client {
    pub db_client: Postgrest,
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}

impl Client {
    pub fn new() -> Self {
        dotenv().ok(); // Load .env file

        // In a real application, these would come from environment variables
        let supabase_url = "http://localhost:8000/rest/v1";
        let supabase_anon_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyAgCiAgICAicm9sZSI6ICJzZXJ2aWNlX3JvbGUiLAogICAgImlzcyI6ICJzdXBhYmFzZS1kZW1vIiwKICAgICJpYXQiOiAxNjQxNzY5MjAwLAogICAgImV4cCI6IDE3OTk1MzU2MDAKfQ.DaYlNEoUrrEn2Ig7tqibS-PHK5vgusbcbo7X36XVt4Q";

        let db_client = Postgrest::new(supabase_url).insert_header("apikey", supabase_anon_key);

        Self { db_client }
    }
}

// Create a global instance of Client wrapped in Arc<Mutex<>>
static CLIENT: OnceLock<Arc<Mutex<Client>>> = OnceLock::new();

// Initialize the global client
pub fn get_client() -> &'static Arc<Mutex<Client>> {
    CLIENT.get_or_init(|| Arc::new(Mutex::new(Client::new())))
}
