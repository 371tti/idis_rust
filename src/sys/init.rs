
use std::collections::HashSet;

use bytes::buf::UninitSlice;
use chrono::Duration;
use mime_guess::{mime, Mime};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[derive(Clone)]
pub struct AppConfig {
    pub default_path: String,
    pub streaming_chunk_size: usize,
    pub chunked_threshold_size: u64,
    pub payload_max_size: usize,
    pub ws_heartbeat_interval: u64,
    pub ws_client_timeout: u64,
    pub server_bind: String,
    pub server_backlog: u32,
    pub server_workers: usize,
    pub server_name_host: String,
    pub server_cluster_enable: bool,
    pub server_cluster_lock_content_len: u64,
    pub server_id: u16,
    pub server_supported_content_types: Vec<Mime>,
    pub mongoDB_addr: String,
    pub mongoDB_name: String,
    pub db_user_collection_name: String,
    pub session_life_time_server: Duration,
    pub session_life_time_client: Duration,
    pub session_len_byte: usize,
    pub session_default_capacity: usize,
    pub session_rng: ChaCha20Rng,
    pub api_key_len_byte: usize,
    pub ruid_rng: ChaCha20Rng,
    pub ruid_prefix: Ruid_prefix_templates,
}

impl AppConfig {
    pub fn new() -> Self {
        let app_config = AppConfig  {
            default_path: r"C:/".to_string(),
            streaming_chunk_size: 1 * 1024 * 1024,
            chunked_threshold_size: 16 * 1024 * 1024 * 1024,
            payload_max_size: 1 * 1024 * 1024,
            ws_heartbeat_interval: 5,
            ws_client_timeout: 30,
            server_bind: "0.0.0.0:83".to_string(),
            server_backlog: 512,
            server_workers: 16,
            server_name_host: "idis-rust-ver0.1.0".to_string(),
            server_cluster_enable: false,
            server_cluster_lock_content_len: 1024,
            server_id: 0xffff,
            server_supported_content_types: AppConfig::supported_content_types(),
            mongoDB_addr: "mongodb://192.168.0.48:27017/?directConnection=true&serverSelectionTimeoutMS=2000&appName=mongosh+2.2.5".to_string(),
            mongoDB_name: "Idis-rust-dev".to_string(),
            db_user_collection_name: "users".to_string(),
            session_life_time_server: Duration::days(365),
            session_life_time_client: Duration::days(365),
            session_len_byte:128,
            session_default_capacity: 100000,
            session_rng: ChaCha20Rng::from_entropy(),
            api_key_len_byte: 32,
            ruid_rng: ChaCha20Rng::from_entropy(),
            ruid_prefix: Ruid_prefix_templates::new(),
        };
        app_config
    }

    fn supported_content_types() -> Vec<Mime> {
        let mut content_types = Vec::new();
        content_types.push(mime::TEXT_PLAIN);
        content_types.push(mime::TEXT_HTML);
        content_types.push(mime::TEXT_CSS);
        content_types.push(mime::TEXT_JAVASCRIPT);
        content_types.push(mime::TEXT_XML);
        content_types.push(mime::APPLICATION_JSON);
        content_types.push(mime::APPLICATION_OCTET_STREAM);
        content_types.push(mime::IMAGE_JPEG);
        content_types.push(mime::IMAGE_PNG);
        content_types
    }


}

#[derive(Clone)]
pub struct Ruid_prefix_templates {
    pub USER_ID: u16,
    pub USER_EXAMPLE_ID: u16,
    pub PERM_EVERYONE: u16,
}

impl  Ruid_prefix_templates {
    pub fn new() -> Self {
        Self {
            USER_ID: 0x2101,
            USER_EXAMPLE_ID: 0x2100,
            PERM_EVERYONE: 0x2201,
        }
    }
}