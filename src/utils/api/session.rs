use std::collections::HashMap;

use chrono::Duration;
use rand_chacha::ChaCha20Rng;
use rand::{RngCore, SeedableRng};

use crate::sys::init::AppConfig;
pub struct Session {
    sessions: HashMap<Vec<u8>, Vec<u128>>,
    len: usize,
    life_time_server: Duration,
    life_time_client: Duration,
    rng: ChaCha20Rng,
    
}

impl Session {
    pub fn new(app_config: &AppConfig) -> Self{
        Session {
            sessions: HashMap::with_capacity(app_config.session_default_capacity),
            len: app_config.session_len_byte,
            life_time_server: app_config.session_life_time_server,
            life_time_client: app_config.session_life_time_client,
            rng: app_config.session_rng.clone(),
        }
    }

    pub fn check() {}

    pub fn set() {}

    pub fn generate(&mut self) -> Vec<u8> {
        let mut buffer = vec![0u8; self.len];
        self.rng.fill_bytes(&mut buffer);

        buffer
    }

}