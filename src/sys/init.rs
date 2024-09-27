
use bytes::buf::UninitSlice;
use chrono::Duration;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[derive(Clone)]
pub struct AppConfig {
    pub default_path: String,
    pub streaming_chunk_size: usize,
    pub chunked_threshold_size: u64,
    pub server_bind: String,
    pub server_backlog: u32,
    pub server_workers: usize,
    pub server_name_host: String,
    pub server_cluster_enable: bool,
    pub server_cluster_lock_content_len: u64,
    pub server_id: u16,
    pub mongoDB_addr: String,
    pub mongoDB_name: String,
    pub session_life_time_server: Duration,
    pub session_life_time_client: Duration,
    pub session_len_byte: usize,
    pub session_default_capacity: usize,
    pub session_rng: ChaCha20Rng,
    pub ruid_rng: ChaCha20Rng,
}

impl AppConfig {
    pub fn new() -> Self {
        let app_config = AppConfig  {
            default_path: r"C:/".to_string(),
            streaming_chunk_size: 1 * 1024 * 1024,
            chunked_threshold_size: 16 * 1024 * 1024 * 1024,
            server_bind: "0.0.0.0:8081".to_string(),
            server_backlog: 512,
            server_workers: 16,
            server_name_host: "idis-rust-ver0.1.0".to_string(),
            server_cluster_enable: false,
            server_cluster_lock_content_len: 1024,
            server_id: 0xffff,
            mongoDB_addr: "mongodb://192.168.0.48:27017/?directConnection=true&serverSelectionTimeoutMS=2000&appName=mongosh+2.2.5".to_string(),
            mongoDB_name: "Idis-rust-dev".to_string(),
            session_life_time_server: Duration::days(365),
            session_life_time_client: Duration::days(365),
            session_len_byte:128,
            session_default_capacity: 100000,
            session_rng: ChaCha20Rng::from_entropy(),
            ruid_rng: ChaCha20Rng::from_entropy(),
        };
        app_config
    }
}