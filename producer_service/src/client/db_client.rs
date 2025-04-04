use dotenv::dotenv;
use postgrest::Postgrest;
use std::env;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};
pub struct Client {
    pub db_client: Postgrest,
}

// we will not be utilizing this in that manner
unsafe impl Send for Client {}
unsafe impl Sync for Client {}

impl Client {
    pub fn new() -> Self {
        dotenv().ok(); // Load .env file

        let supabase_url = env::var("SUPABASE_URL")
        .unwrap_or_else(|_| "http://kong:8000/rest/v1".to_string());
        let supabase_anon_key =
            env::var("ANON_KEY").expect("SUPABASE_ANON_KEY must be set");

        let db_client = Postgrest::new(&supabase_url).insert_header("apikey", &supabase_anon_key);

        Self { db_client }
    }
}
// Create a global instance of Client wrapped in Arc<Mutex<>>
static CLIENT: OnceLock<Arc<Mutex<Client>>> = OnceLock::new();

// Initialize the global client
pub fn get_client() -> &'static Arc<Mutex<Client>> {
    CLIENT.get_or_init(|| Arc::new(Mutex::new(Client::new())))
}
